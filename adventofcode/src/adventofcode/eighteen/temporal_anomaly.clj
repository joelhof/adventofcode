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
      ;    (println seen (first seq))
      (if (contains? seen (first seq))
        (first seq)
        (recur (rest seq) (conj seen (first seq)))
        )
      )

(def dayOnePart2
    (println "Day 1, part 2" (findFirstRepeat (reductions + 0 (cycle (freq))) #{}))
)