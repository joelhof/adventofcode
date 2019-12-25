(ns adventofcode.nineteen.int-computer)

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

(defn get-opcode
  [instr]
  (mod instr 100)
  )

(defmulti pointer-instr (fn [pointer program] (get-opcode
                                               (nth program pointer))))
(defmethod pointer-instr 1 [pointer program]
  ;(println "+ pointer at:" pointer)
  (let [[i j k] (subvec program (inc pointer) (+ pointer 4))
        new-pointer (+ pointer 4)
        modes (parameter-modes pointer program 3)]
    [new-pointer (assoc program k (+ (parameter-value (nth modes 0 0) i program)
                                     (parameter-value (nth modes 1 0) j program)))])
  )

(defmethod pointer-instr 2 [pointer program]
  ;(println "* pointer at:" pointer)
  (let [[i j k] (subvec program (inc pointer) (+ pointer 4))
        new-pointer (+ pointer 4)
        modes (parameter-modes pointer program 3)]
    [new-pointer (assoc program k (* (parameter-value (nth modes 0 0) i program)
                                     (parameter-value (nth modes 1 0) j program)))]
    )
  )

(defmethod pointer-instr 99 [pointer program]
  ; XXX think about how to signal program halt.
  ;(println "Halting! pointer at:" pointer)
  [pointer program])

(defmethod pointer-instr 3 [pointer program]
  ; Store input in memory
  ;(println "Storing " @input "@ pos " pointer)
  (let [i (nth program (inc pointer))]
    [(+ pointer 2) (assoc program i @input)])
  )

(defmethod pointer-instr 4 [pointer program]
  ; Output value from memory
  ;(println "output instruction...")
  (let [new-pointer (inc pointer)
        modes (parameter-modes pointer program 1)
        i (nth program new-pointer)]
    (reset! output (parameter-value (nth modes 0 0) i program))
    [(inc new-pointer) program]
    )
  )

(defmethod pointer-instr 5 [pointer program]
  ; GOTO if non-zero
  (let [[i j] (subvec program (inc pointer) (+ 3 pointer))
        modes (parameter-modes pointer program 2)
        firstParam (parameter-value (nth modes 0 0) i program)
        new-pointer (if (not (zero? firstParam))
                      (parameter-value (nth modes 1 0) j program) (+ 3 pointer)) ]
    [new-pointer program])
  )

(defmethod pointer-instr 6 [pointer program]
  ; GOTO if non-zero
  (let [[i j] (subvec program (inc pointer) (+ 3 pointer))
        modes (parameter-modes pointer program 2)
        firstParam (parameter-value (nth modes 0 0) i program)
        new-pointer (if (zero? firstParam)
                      (parameter-value (nth modes 1 0) j program) (+ 3 pointer))]
    [new-pointer program]))

(defn predicate
  [f pointer program]
  ; f takes 2 args returning a boolean.
  ; If f returns true, store 1 at instructions 3:rd parameter. 
  (let [params (subvec program (inc pointer) (+ 4 pointer))
        modes (parameter-modes pointer program 3)
        values (get-parameter-values modes params program)]
    [(+ 4 pointer) (assoc program (last params)
                          (if (f (first values) (second values))
                            1
                            0))])
  )

(defmethod pointer-instr 7 [pointer program]
  ; LESS THAN
  ;(println "LESS THAN instruction")
  (predicate < pointer program))

(defmethod pointer-instr 8 [pointer program]
  ; EQUALS
  ;(println "EQUALS instruction")
  (predicate = pointer program))

(defn run
  ([program] (run 0 program))
  ([pointer program]
   ;(println pointer program)
   (let [[new-pointer new-prog] (pointer-instr pointer program)]
     (if (= pointer new-pointer)
       new-prog
       (recur new-pointer new-prog))
     )
   ))

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