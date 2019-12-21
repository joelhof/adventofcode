(ns adventofcode.nineteen.int-computer)

(defn digits [number]
  (rseq (mapv #(mod % 10) (take-while pos? (iterate #(quot % 10) number)))))

(defmulti parameter-value (fn [mode & params] mode))
(defmethod parameter-value 0 [_ i instructions] (nth instructions i))
(defmethod parameter-value 1 [_ i] i)

;(def pointer (atom 0))

(def input (atom 0))
(def output (atom 0))

(defn get-opcode
  [instr]
  (last (digits instr))
  )

(defmulti pointer-instr (fn [pointer program] (get-opcode
                                               (nth program pointer))))
(defmethod pointer-instr 1 [pointer program]
  ;(println "+ pointer at:" pointer)
  (let [[i j k] (subvec program (inc pointer) (+ pointer 4))
        new-pointer (+ pointer 4)]
    [new-pointer (assoc program k (+ (nth program i) (nth program j)))])
  )
(defmethod pointer-instr 2 [pointer program]
  ;(println "* pointer at:" pointer)
  (let [[i j k] (subvec program (inc pointer) (+ pointer 4))
        new-pointer (+ pointer 4)]
    [new-pointer (assoc program k (* (nth program i) (nth program j)))]
    )
  )
(defmethod pointer-instr 99 [pointer program]
  ; XXX think about how to signal program halt.
  (println "Halting! pointer at:" pointer)
  [pointer program])
(defmethod pointer-instr 3 [pointer program]
  ; Store input in memory
  (let [i (nth program (inc pointer))]
    [(+ pointer 2) (assoc program i @input)])
  )
(defmethod pointer-instr 4 [pointer program]
  ; Output value from memory
  (let [new-pointer (inc pointer)
        i (nth program new-pointer)]
    (reset! output (nth program i))
    [(inc new-pointer) program]
    )
  )
(defn run
  ([program] (run 0 program))
  ([pointer program]
   (println pointer program)
   (let [[new-pointer new-prog] (pointer-instr pointer program)]
     (if (= pointer new-pointer)
       new-prog
       (recur new-pointer new-prog))
     )
   ))

(map #(hash-map :mode %1, :value %2) [4 3 4] [1 0])
({:value 1, :mode 4} {:value 0, :mode 3}) 


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