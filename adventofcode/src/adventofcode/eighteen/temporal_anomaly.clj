(ns adventofcode.eighteen.temporal-anomaly
    (:require [clojure.string :as string])
    (:require [clojure.set :as clojure.set])
    (:import (java.time.format DateTimeFormatter) (java.time LocalDateTime)))

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

(defn hammingDistance [s1 s2]
      (count (filter false? (map = s1 s2)))
)

(defn compareIds
  [s coll]
   (->> coll
       (remove #(= s %) ,,,)
       (filter #(= 1 (hammingDistance s %)) ,,,)
       (first ,,,)
       (map vector s ,,,)
       (filter #(= (first %) (second %)) ,,,)
       (map first ,,,)
   )
)

(defn commonLetters
  [strings]
  (->> strings
    (map #(compareIds % strings) ,,,)
    (filter #(not (empty? %)) ,,,)
    (map string/join ,,,)
    (first ,,,)
  )
)

(def dayTwoPart1
  (println "Day 2, part 1:"
    (letterCountChecksum (string/split-lines (slurp "resources/eighteen/dayTwo.txt"))))
)

(def dayTwoPart2
  (println "Day 2, part 2:"
    (commonLetters (string/split-lines (slurp "resources/eighteen/dayTwo.txt"))))
)

(defn points
  [[x y] [deltaX deltaY]]
  (apply concat
         (map #(map vector (range y (+ y deltaY)) (repeat %)) (range x (+ x deltaX))))
)

(defn area
  [id x y deltaX deltaY]
  (reduce #(assoc %1 %2 [id]) {} (points [x y] [deltaX deltaY]))
)

(defn claimMap
  [claims]
  (->> claims
       (map #(apply area %) ,,,)
       (apply merge-with into ,,,)
    )
)

(defn countClaimConflicts
  [claims]
  (->> claims
       (claimMap ,,,)
       (filter #(< 1 (count (second %))) ,,,)
       (count ,,,)
  )
)

(defn findIntactClaim
  [claims]
  (let [claimIds (->> (claimMap claims)
                 (vals ,,,)
                 )]
       (clojure.set/difference (set (flatten claimIds))
                               (set (flatten (filter #(< 1 (count %)) claimIds))))
  )
)

(defn getCoords
      [strings]
      [(mapv read-string (string/split (first strings) #","))
       (mapv read-string (string/split (second strings) #"x"))]
)

(defn getArgs
      [s]
      [(first s) (getCoords (string/split (second s) #":"))]
)

(defn readClaim
  [s]
  (-> s
      (string/replace ,,, " " "")
      (string/split ,,, #"@")
      (getArgs ,,,)
      (flatten ,,,)
      (vec ,,,)
  )
)

(defn dayThreePart1 []
  (->> (slurp "resources/eighteen/dayThree.txt")
       (string/split-lines )
       (mapv readClaim )
       (countClaimConflicts )
       (println "Day 3, part 1:" ,,,)
       )
)

(defn dayThreePart2 []
  (->> (slurp "resources/eighteen/dayThree.txt")
       (string/split-lines )
       (mapv readClaim )
       (findIntactClaim )
       (println "Day 3, part 2:" ,,,)
       )
)

(defn parseDateTime
  [datetime]
  (LocalDateTime/parse datetime (DateTimeFormatter/ofPattern "yyyy-MM-dd HH:mm"))
)

(defn extractDateTimeStr
  [s]
   (second (re-find #"\[(.*)\]" s))
)

(defn dayFourPart1 []
      (->> "resources/eighteen/dayFour.txt"
        (slurp )
        (string/split-lines )
        (sort-by #(parseDateTime (extractDateTimeStr %)) )
      )
)