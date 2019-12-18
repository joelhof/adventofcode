(ns adventofcode.nineteen.stranded-santa
  (:require [clojure.string :as string]))

(defn module-fuel
  [mass] (- (Math/floor (/ mass 3)) 2))

(defn parse-day-one-input
  []
  (as-> "resources/nineteen/dayOne.txt" masses
   (slurp masses)
   (string/split-lines masses)
   (map #(Integer. %) masses)))

(defn day-one-part-one
  [] (->> (parse-day-one-input)
      (map module-fuel)
      (reduce +)))

(defn recursive-fuel
  ([mass] (recursive-fuel mass 0))
  ([mass sum] (let [fuel (module-fuel mass)]
               (if (neg? fuel)
                sum
                (recur fuel (+ sum fuel))))))

(defn day-one-part-two [] (->> (parse-day-one-input)
                           (map recursive-fuel)
                           (reduce +)))

(defn opcode-sum
  [[i j k] data]
  (println "opcode-sum " i j k)
  (assoc data k (+ (nth data i) (nth data j))))

(defn opcode-multiply
  [[i j k] data]
  (println "opcode-multiply " i j k)
  (assoc data k (* (nth data i) (nth data j))))

(defn int-code-step
  ([data] (int-code-step 0 data))
  ([instruction data]
   (println "next instruction " instruction)
   (case (nth data instruction)
     99 data
     1 (->> data
            (opcode-sum (subvec data (inc instruction) (+ instruction 4)) ,,,)
            (int-code-step (+ instruction 4) ,,,))
     2 (->> data
           (opcode-multiply (subvec data (inc instruction) (+ instruction 4)) ,,,)
           (int-code-step (+ instruction 4) ,,,))))) 

(defn prepare-input
  []
  (-> (mapv #(Integer. %) (-> "resources/nineteen/dayTwo.txt"
                              (slurp)
                              (string/trim)
                              (string/split ,,, #",")))
      (assoc ,,, 1 12)
      (assoc ,,, 2 2)))

(defn day-two-part-one
  [] (first (int-code-step (prepare-input))))

(defn digits [number] (rseq (mapv #(mod % 10) (take-while pos? (iterate #(quot % 10) number)))))

(defn doubles? [digits] (> (count digits) (count (distinct digits))))

(defn monotonic? [digits] (= (sort digits) digits))

(defn day-four-part-one []
	(->> (range 178416 676462)
	(map digits ,,,)
	(filter doubles? ,,,)
	(filter monotonic? ,,,)
	(count ,,,))
)

(defn strict-doubles? [digits] (contains? (set (vals (frequencies digits))) 2))

(defn day-four-part-two []
	(->> (range 178416 676462)
	(map digits ,,,)
	(filter strict-doubles? ,,,)
	(filter monotonic? ,,,)
	(count ,,,))
)

(defn to-int-seq
  [file]
  (->> file
       (slurp ,,,)
       (string/trim ,,,)
       (map #(Character/digit % 10) ,,,)
       )
  )

(defn image-checksum
  [image]
  (let [freqs (frequencies image)]
    (* (get freqs 1 0) (get freqs 2 0)))
  )

(defn day-eight-part-one []
  (->>  "resources/nineteen/dayEight.txt"
       (to-int-seq ,,,)
       (partition (* 25 6) ,,,)
       (sort-by #(get (frequencies %) 0 0) ,,,)
       (first ,,,)
       (image-checksum ,,,)
       )
)

(defn pixel-value
  [coll m n] 
  (->> coll
       (take-nth (* m n) ,,,)
       (filter #(not (= 2 %) ,,,))
       (first ,,,))
  )

(defn merge-layers [m n layers]
  (->> (iterate rest layers)
       (take (* m n) ,,,)
       (map #(pixel-value % m n) ,,,)
       (partition m ,,,))
  )

(defn display [image] 
  (doseq [row image] (println row)))

(defn day-eight-part-two
  []
  (->> "resources/nineteen/dayEight.txt"
       (to-int-seq ,,,)
       (merge-layers  25 6 ,,,) 
       (display ,,,))
  )

(defn parse-reaction
  [reaction]
  (->> (string/split reaction #",")
       (map string/trim ,,,)
       (map #(string/split % #" ") ,,,)
       (map #(hash-map (keyword (second %)) (Integer. (first %))) ,,,)
       )
  )

(defn parse-reaction-str
  [input]
  (->> (string/split-lines input)
       (map #(string/split % #"=>") ,,,)
       (map #(map parse-reaction %) ,,,)
       (map #(hash-map :lhs (first %) :rhs (second %) :expr (first (keys (first (second %))))) ,,,)
       (group-by :expr ,,,))
  )
; Sum up all 'primitive' reaction results,
; i.e those directly converted from ORE.
; i.e where LHS is 'X ORE' and RHS is 'Y PRIMITIVE'
; Summing can be done recursively

(defn multiplier
  [base x resultant]
  (* base (+ (quot x resultant) (if (= 0 (mod x resultant)) 0 1)))
  )

(defn expanded-multiplier
  [x y]
  (+ (quot x y) (if (= 0 (mod x y)) 0 1)))
;noppannoppansson

; Use breath first search to expand the expressions

(defn expand-expr 
  [lookup-table expand]
  ; lookup expression
  (let [expr-key (first (keys expand))
        expr (lookup-table expr-key)
        multiplier (expanded-multiplier
                    (expand expr-key) (expr :increment))]
    (map #(update % (first (keys %)) * multiplier) (expr :lhs)))
  )