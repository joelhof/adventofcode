(ns adventofcode.calendar
  (:require [adventofcode.core :as core])
)

(def input 
  (slurp "resources/inputDayTwo.txt")
)

(defn dayTwoResult
  []
  (println "Day Two, calculating checksum...")
  (core/checkSum (map (fn [x] (map #(Integer/valueOf %) x))
                      (map #(clojure.string/split % #"\t")
                           (clojure.string/split-lines input))))
)
