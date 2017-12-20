(ns adventofcode.calendar
  (:require [adventofcode.core :as core])
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
