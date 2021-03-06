(ns adventofcode.calendar
  (:require [adventofcode.core :as core :refer [r] :rename {r r}]
            [clojure.string :as string])
)

(def input 
  (slurp "resources/inputDayTwo.txt")
)

(defn spreadSheet
  [input]
  (map (fn [x] (mapv #(Integer/valueOf %) x))
                       (map #(clojure.string/split % #"\t")
                            (clojure.string/split-lines input)))
)  

(defn dayTwoResult
  []
  (println "Day Two, part 1, calculating checksum...")
  (core/spreadSheetCheckSum core/rowCheckSum (spreadSheet input))
)

(defn dayTwoResultPart2
  []
  (println "Day Two, part 2, calculating checksum...")
  (core/spreadSheetCheckSum core/rowModuloCheckSum (spreadSheet input))
)

(defn dayFourPart1
  []
  (println "Day Four, part 1, counting valid passphrases...")
  (reduce + (map core/isPassphraseValid?
                 (string/split-lines (slurp "resources/day4input1.txt"))))
)

(defn dayFourPart2
  []
  (println "Day Four, part 2, counting valid passphrases...")
  (reduce + (map core/isPassphraseValid?
                 (string/split-lines (slurp "resources/day4input1.txt"))))
)

(defn dayFivePart1
  []
  (println "Day Five, part 1, escaping maze...")
  (core/jump
    (vec (map #(Integer/valueOf %)
              (string/split-lines (slurp "resources/day5Input1.txt")))))
)

(defn daySixPart1
  []
  (println "Day Six, part 1, reallocating memory...")
  (core/reallocate (vec (map #(Integer/valueOf %)
                             (string/split
                               (string/trim-newline
                                 (slurp "resources/day6part1.txt")) #"\t"))))
  )

(defn daySevenPart1
  []
  (println "Day Seven, part 1, finding tree root...")
  (:name (core/findRoot (string/split-lines (slurp "resources/day7input1.txt"))))
)

(defn daySevenPart2
  []
  (println "Day Seven, part 2, finding unbalanced node...")
  (let [unbalancedNode (core/balanceTree (string/split-lines (slurp "resources/day7input1.txt")))
        ]
       (println "Unbalanced node needs to weigh:")
       (- (core/intWeight unbalancedNode) (:diff unbalancedNode))
       )
      )

(defn dayEightPart1
      []
      (println "Day eight, part 1, find max register value")
      (core/currentMaxRegisterValue (string/split-lines (slurp "resources/day8input1.txt"))))

(defn dayEightPart2
      []
      (println "Day eight, part 2, find all time max register value")
      (core/allTimeMaxRegisterValue (string/split-lines (slurp "resources/day8input1.txt"))))