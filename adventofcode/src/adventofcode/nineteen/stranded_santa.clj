(ns adventofcode.nineteen.stranded-santa
  (:require [clojure.string :as string])
  (:require [clojure.core.async :as async])
  (:use [adventofcode.nineteen.int-computer :as integer-computer :only (run)]))

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

(defn prepare-input
  [file]
  (-> (mapv #(Integer. %) (-> file
                              (slurp)
                              (string/trim)
                              (string/split ,,, #",")))
      ))

(defn run-with-input
  [input noun verb]
  (first (integer-computer/run (-> input
                                   (assoc ,,, 1 noun)
                                   (assoc ,,, 2 verb)))))

(defn day-two-part-one
  [] (run-with-input (prepare-input "resources/nineteen/dayTwo.txt") 12 2)
  )

(def combos 
  (->> (range 0 100)
       (map (fn [x] (map #(vector x %) (range 0 100))) ,,,)
       (flatten ,,,)
       (partition 2 ,,,)
       (shuffle ,,,) ; increases performance slightly.
       ) 
)

(defn day-two-part-two []
  (let [input-program (prepare-input "resources/nineteen/dayTwo.txt")]
    (println "Finding input noun and verb producing 19690720...")
    (reduce (fn [previous [noun verb]]
              (if (= (first previous) 19690720)
                (reduced previous)
                [(run-with-input input-program noun verb) (+ (* 100 noun) verb)])) [0 0] combos))
  )

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

(defn print-watcher [channel]
  (async/go-loop []
    (when-let [value (async/<! channel)]
      (println "LOG:" value) (recur)
      )
    ))

(defn day-five-part-one
  []
  (let [in-chan (async/chan 1)
        out-chan (async/chan 20)]
    (async/>!! in-chan 1)
    (print-watcher out-chan)
    (-> (prepare-input "resources/nineteen/dayFive.txt")
        (integer-computer/run in-chan out-chan))
    )
  )

(defn day-five-part-two []
  (let [in-chan (async/chan 1)
        out-chan (async/chan 20)]
    (async/>!! in-chan 5)
    (print-watcher out-chan)
    (-> (prepare-input "resources/nineteen/dayFive.txt")
        (integer-computer/run in-chan out-chan)))
  )

(defn swap-phases
  [i j phases]
  (assoc phases i (phases j) j (phases i))
  )

(defn cycle-phases
  [phases]
  (map #(subvec (vec (take (* 2 (count phases))
                            (cycle phases))) % (+ % (count phases))) (range 0 (count phases))))

(defn generate-phases
  [phase phase-set]
  (->> (repeatedly (count phase-set) #(vec phase-set))
       (map-indexed #(swap-phases (.indexOf %2 phase) %1 %2) ,,,)
       (map cycle-phases ,,,)
       (reduce concat ,,,))
  )

(defn phase-settings
  [m n]
  (->> (range m n)
       (map #(generate-phases % (range m n)) ,,,)
       (apply concat ,,,)
       (distinct ,,,))
  )

(defn amplify
  [program phase-settings]
  (loop [phase-settings phase-settings
         input 0]
    ;(prn phase-settings input)
    (if (empty? phase-settings)
      input
      (recur (rest phase-settings)
             (integer-computer/amplify!
              (first phase-settings) input program))
      )
    )
  )

(defn day-seven-part-one []
  (println "Finding optimial thruster phase settings...")
  (->> (phase-settings 0 5)
       (map (juxt identity (partial amplify (prepare-input "resources/nineteen/daySeven.txt"))) ,,,)
       (sort-by second ,,,)
       (reverse ,,,)
       (first ,,,)
       )
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
       (mapv #(hash-map (keyword (second %)) (Integer. (first %))) ,,,)
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
(defmulti expand-expr (fn [lookup-table expand] (first (keys expand))))
(defmethod expand-expr :ORE [lookup-table expand]
   (println "expanding :ORE" expand)
   expand)
(defmethod expand-expr :default [lookup-table expand]
  (println "expanding" expand)
  (let [expr-key (first (keys expand))
        expr (get-in lookup-table [expr-key 0])
        multiplier (expanded-multiplier
                    (expand expr-key)
                    (get-in expr [:rhs 0 expr-key]))]
    (map #(update % (first (keys %)) * multiplier) (expr :lhs)))
  )

(defn expand [lookup-table key]
  (->> (get-in lookup-table [key 0 :lhs])
       (map #(expand-expr lookup-table %))
       (flatten)
       (apply merge-with +)
       ;(mapv #(hash-map (key %) (val %)))
       )
  )

(def lookup-table1 (parse-reaction-str "157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT"))