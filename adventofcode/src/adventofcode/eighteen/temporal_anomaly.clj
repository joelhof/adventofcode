(ns adventofcode.eighteen.temporal-anomaly
    (:require [clojure.string :as string]))

(defn freq
      []
      (map read-string
           (string/split-lines
             (slurp "resources/eighteen/dayOne.txt")))
)

(def dayOnePart1
  (println "2018: Day 1, part 1: " (reduce + 0 (freq)))
)

(defn findFirstRepeat [seq seen]
      (if (contains? seen (first seq))
        (first seq)
        (recur (rest seq) (conj seen (first seq)))
        )
      )

(def dayOnePart2
    (println "Day 1, part 2" (findFirstRepeat (reductions + 0 (cycle (freq))) #{}))
)

(defn hasOccurence
  [s pred]
      (if (empty? (filter pred (frequencies s)))
     0
     1)
)

(defn letterCount
      [s]
      [(hasOccurence s #(= (val %) 2)) (hasOccurence s #(= (val %) 3))]
      )

(defn letterCountChecksum
  [coll]
      (->> coll
           (map letterCount ,,,)
           (reduce #(conj [] (+ (first %1) (first %2)) (+ (second %1) (second %2))) [0 0] ,,,)
           (reduce * ,,,)
           )
      )

(def dayTwoPart1
  (println (letterCountChecksum (string/split-lines (slurp "resources/eighteen/dayTwo.txt"))))
  )