(ns adventofcode.nineteen.int-computer
  (:require [clojure.core.async :as async]))

(def LOG (atom false))

(defn to-log [& more]
  (.write *out* (str (clojure.string/join " " more) "\n")))

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
  (when @LOG (to-log (:host m) "+ pointer at:" pointer))
  (let [[i j k] (subvec program (inc pointer) (+ pointer 4))
        new-pointer (+ pointer 4)
        modes (parameter-modes pointer program 3)]
    (update-state m new-pointer k (+ (parameter-value (nth modes 0 0) i program)
                                     (parameter-value (nth modes 1 0) j program)))
    )
  )

(defmethod pointer-instr 2 [{:keys [pointer program] :as m}]
  (when @LOG (to-log (:host m) "* pointer at:" pointer))
  (let [[i j k] (subvec program (inc pointer) (+ pointer 4))
        new-pointer (+ pointer 4)
        modes (parameter-modes pointer program 3)]
    (update-state m new-pointer k (* (parameter-value (nth modes 0 0) i program)
                                     (parameter-value (nth modes 1 0) j program)))
    )
  )

(defmethod pointer-instr 99 [{:keys [pointer out-chan] :as m}]
  (to-log (:host m) "Halting! pointer at:" pointer)
  (async/close! out-chan)
  (assoc m :exit true)
  )

(defmethod pointer-instr 3 [{:keys [pointer program in-chan] :as m}]
  ; Store input in memory
  (when @LOG (to-log (:host m) "Reading input..." pointer))
  (let [i (nth program (inc pointer))
        input-value (async/<!! in-chan)]
    (to-log (:host m) "Input" input-value "recieved. Proceeding...")
    (update-state m (+ pointer 2) i input-value)
    ))

(defmethod pointer-instr 4 [{:keys [pointer program out-chan] :as m}]
  ; Output value from memory
  ;(when @LOG 
    (to-log (:host m) "output instruction..." pointer)
   ; )
  (let [new-pointer (inc pointer)
        modes (parameter-modes pointer program 1)
        i (nth program new-pointer)]
    (async/put! out-chan (parameter-value (nth modes 0 0) i program))
    (assoc m :pointer (+ pointer 2))
    ))

(defmethod pointer-instr 5 [{:keys [pointer program] :as m}]
  ; GOTO if non-zero
  (when @LOG (to-log (:host m) "GOTO if not zero" pointer))
  (let [[i j] (subvec program (inc pointer) (+ 3 pointer))
        modes (parameter-modes pointer program 2)
        firstParam (parameter-value (nth modes 0 0) i program)
        new-pointer (if (not (zero? firstParam))
                      (parameter-value (nth modes 1 0) j program) (+ 3 pointer))]
    (assoc m :pointer new-pointer)
    )
  )

(defmethod pointer-instr 6 [{:keys [pointer program] :as m}]
  ; GOTO if zero
  (when @LOG (to-log (:host m) "GOTO if zero" pointer))
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
    )

(defmethod pointer-instr 7 [state]
  ; LESS THAN
  (when @LOG (to-log (:host state) "LESS THAN" (:pointer state)))
  (predicate < state))

(defmethod pointer-instr 8 [state]
  ; EQUALS
  (when @LOG (to-log (:host state) "EQUALS" (:pointer state)))
  (predicate = state))

(defn run
  ([program] (run 0 {:pointer 0 :program program :host :localhost} ))
  ([program input< output> name] (run 0 {
                                         :pointer 0
                                         :program program 
                                         :in-chan input< 
                                         :out-chan output>
                                         :host name}))
  ([pointer program]
   (to-log pointer program)
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
    (run program in-chan out-chan :localhost)
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