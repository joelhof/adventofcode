(ns adventofcode.nineteen.int-computer
  (:require [clojure.core.async :as async]))

(defn digits [number]
  (rseq (mapv #(mod % 10) (take-while pos? (iterate #(quot % 10) number)))))

(defmulti parameter-value (fn [mode & params] mode))
(defmethod parameter-value 0 [m i program] (nth program i))
(defmethod parameter-value 1 [m i & program] i)

(defn parameter-modes
  [pointer program n]
  (let [param-modes (drop-last 2 (digits (nth program pointer)))]
    (reverse (concat 
              (replicate (- n (count param-modes)) 0)
              param-modes))
    )
  )

(defn get-parameter-values
  [modes params program]
  (map #(parameter-value (first %) (second %) program)
       (partition 2 (interleave modes params))))

;(def pointer (atom 0))

(def input (atom 0))
(def output (atom 0))

(def input-chan (async/chan 2))
(def output-chan (async/chan 1))

(defn get-opcode
  [instr]
  (mod instr 100)
  )

(defn update-state [m p i value]
  (-> m
      (assoc-in ,,, [:pointer] p)
      (assoc-in ,,, [:program i] value)))

(defmulti pointer-instr (fn [{:keys [pointer program]}] (get-opcode
                                               (nth program pointer))))
(defmethod pointer-instr 1 [{:keys [pointer program] :as m}]
  (println "+ pointer at:" pointer)
  (let [[i j k] (subvec program (inc pointer) (+ pointer 4))
        new-pointer (+ pointer 4)
        modes (parameter-modes pointer program 3)]
    (update-state m new-pointer k (+ (parameter-value (nth modes 0 0) i program)
                                     (parameter-value (nth modes 1 0) j program)))
    )
  )

(defmethod pointer-instr 2 [{:keys [pointer program] :as m}]
  (println "* pointer at:" pointer)
  (let [[i j k] (subvec program (inc pointer) (+ pointer 4))
        new-pointer (+ pointer 4)
        modes (parameter-modes pointer program 3)]
    (update-state m new-pointer k (* (parameter-value (nth modes 0 0) i program)
                                     (parameter-value (nth modes 1 0) j program)))
    )
  )

(defmethod pointer-instr 99 [{:keys [pointer out-chan] :as m}]
  ; XXX think about how to signal program halt.
  (println "Halting! pointer at:" pointer)
  (async/close! out-chan)
  (assoc m :exit true)
  )

(defmethod pointer-instr 3 [{:keys [pointer program in-chan] :as m}]
  ; Store input in memory
  (println "Reading input..." pointer)
  (let [i (nth program (inc pointer))
        input-value (async/<!! in-chan)]
    (println "Input" input-value "recieved. Proceeding...")
    (update-state m (+ pointer 2) i input-value)
    ))

(defmethod pointer-instr 4 [{:keys [pointer program out-chan] :as m}]
  ; Output value from memory
  (println "output instruction..." pointer)
  (let [new-pointer (inc pointer)
        modes (parameter-modes pointer program 1)
        i (nth program new-pointer)]
    (async/put! out-chan (parameter-value (nth modes 0 0) i program))
    (assoc m :pointer (+ pointer 2))
    ))

(defmethod pointer-instr 5 [{:keys [pointer program] :as m}]
  ; GOTO if non-zero
  (let [[i j] (subvec program (inc pointer) (+ 3 pointer))
        modes (parameter-modes pointer program 2)
        firstParam (parameter-value (nth modes 0 0) i program)
        new-pointer (if (not (zero? firstParam))
                      (parameter-value (nth modes 1 0) j program) (+ 3 pointer))]
    (assoc m :pointer new-pointer)
    )
  )

(defmethod pointer-instr 6 [{:keys [pointer program] :as m}]
  ; GOTO if non-zero
  (let [[i j] (subvec program (inc pointer) (+ 3 pointer))
        modes (parameter-modes pointer program 2)
        firstParam (parameter-value (nth modes 0 0) i program)
        new-pointer (if (zero? firstParam)
                      (parameter-value (nth modes 1 0) j program) (+ 3 pointer))]
    (assoc m :pointer new-pointer)))

(defn predicate
  [f {:keys [pointer program] :as m}]
  ; f takes 2 args returning a boolean.
  ; If f returns true, store 1 at instructions 3:rd parameter. 
  (let [params (subvec program (inc pointer) (+ 4 pointer))
        modes (parameter-modes pointer program 3)
        values (get-parameter-values modes params program)]
    (update-state m (+ 4 pointer) (last params) (if (f
                                                     (first values) (second values))
                                                  1
                                                  0)))
   ;  [(+ 4 pointer) (assoc program (last params)
   ;                        (if (f (first values) (second values))
   ;                          1
   ;                          0))])
    )

(defmethod pointer-instr 7 [state]
  ; LESS THAN
  ;(println "LESS THAN instruction")
  (predicate < state))

(defmethod pointer-instr 8 [state]
  ; EQUALS
  ;(println "EQUALS instruction")
  (predicate = state))

(defn run
  ([program] (run 0 {:pointer 0 :program program} ))
  ([program input< output>] (run 0 {
                                    :pointer 0
                                    :program program 
                                    :in-chan input< 
                                    :out-chan output>}))
  ([pointer program]
   ;(println pointer program)
   (let [new-prog (pointer-instr program)]
     (if (:exit new-prog)
       new-prog
       (recur 0 new-prog))
     )
   ))

(defn amplify!
  [phase input-value program]
  (let [in-chan (async/chan 2)
        out-chan (async/chan 1)]
    (async/>!! in-chan phase)
    (async/>!! in-chan input-value)
    (run program in-chan out-chan)
    (async/<!! out-chan))
  ;(reset! input (list phase input-value))
  
  )

(def test-program [3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99])

; (defprotocol Opcode
;   (params [this])
;   (code [this])
;   (exec [[code & params] program])
;   )

; (deftype Add Opcode
;   (params [this] 4)
;   (code [this] 1)
;   (exec [[c i j k] instructions]
;         (assoc instructions k (+ (nth instructions i) (nth instructions j))))
;   )