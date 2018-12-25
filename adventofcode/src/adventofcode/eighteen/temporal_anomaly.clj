(ns adventofcode.eighteen.temporal-anomaly
    (:require [clojure.string :as string])
    (:require [clojure.set :as clojure.set])
    (:import (java.time.format DateTimeFormatter)
             (java.time LocalDateTime)
             (java.time.temporal ChronoUnit))

    )

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

; if string is '#Id begins shift' -> parse id, set current id.
; if string is 'falls asleep' store hh:mm
; if string is 'wakes up' -> calculate and store sleep time in array of size 60.
; assoc sleep time array with guard id.
; keep state in map: {current:#id, sleep: nil or 'hh:mm', sleepTimes:{ #ids: [] }}
; recur until event list is exhausted

(defn getGuard
  [s]
  (re-find #"#\d*" s)
)

(defn inc-default
  [x]
  (inc (or x 0))
)

(defn sleptBetween
      [sleep sleepLength]
      (reduce #(update %1 %2 inc-default) sleep sleepLength)
)

(defn addSleepTime
  [event state]
  (let [
        sleepLength (range (.getMinute (:sleep state))
                        (inc (.getMinute (parseDateTime (extractDateTimeStr event)))))
        newState (update-in state [:sleepTimes (:current state)] sleptBetween sleepLength)
        ]
        (assoc newState :sleep nil)
       )
)

(defn parseEvent
  [event state]
  (cond
    (.contains event "begins shift") (assoc state :current (getGuard event))
    (.contains event "falls asleep") (assoc state :sleep (parseDateTime (extractDateTimeStr event)))
    (.contains event "wakes up") (addSleepTime event state)
    :else (throw (RuntimeException. "Unmatched event type")))
)

(defn parseEvents
  [state events]
  (loop [events events, state state]
    (if (empty? events)
          state
          (recur (rest events) (parseEvent (first events) state))
    )
  )
)

(defn sumSleepTime
      [e]
      (reduce + (vals (val e)))
)

(defn calculateAnswer
      [m]
      (println (first (last (sort-by val (val m)))))
      (println (Integer/valueOf (string/replace (key m) "#" "")))
      (* (first (last (sort-by val (val m))))
         (Integer/valueOf (string/replace (key m) "#" "")))
)

(defn findGuard-by
      [f]
      (->> "resources/eighteen/dayFour.txt"
           (slurp ,,,)
           (string/split-lines ,,,)
           (sort-by #(parseDateTime (extractDateTimeStr %)) ,,,)
           (parseEvents {} ,,,)
           (:sleepTimes ,,,)
           (sort-by f ,,,)
           (last ,,,)
           (calculateAnswer ,,,)
      )
)

(defn dayFourPart1 []
      (println "Day 4, part 1: The Guard ID multiplied by the day is"
               (findGuard-by #(reduce + (vals (val %)))))
)

(defn dayFourPart2 []
      (println "Day 4, part 2: The Guard ID multiplied by the day is"
               (findGuard-by #(last (sort (vals (val %))))))
)

; day five

; scan input char by char
; compare current char to next
; if reaction occurs, throw away current and next
;   and set current to previous
; recur

(defn reacts?
  [a b]
  (and (not (= a b)) (.equalsIgnoreCase (str a) (str b)))
)

(defn pop-or-empty
      [stack]
      (if (empty? stack) (list) (pop stack)))

(defn polymer-reactions
  [stack]
  (loop [unreacted stack
         polymers '()]
    (if (or (= \newline (peek unreacted)) (empty? unreacted))
        polymers
        (if (reacts? (peek unreacted) (peek polymers))
          (recur (pop unreacted) (pop-or-empty polymers))
          ; if no reaction, store head of unreacted in polymers, pop unreacted
          (recur (pop unreacted) (conj polymers (peek unreacted)))
        )
    )
  )
)

(defn format-polymers
  [polymers]
   (-> polymers
       (reverse ,,,)
       (string/join ,,,)
   )
)

(defn alchemical-reduction
  [polymer-str]
  (->> polymer-str
      (seq ,,,)
      (apply list  ,,,)
      (polymer-reactions ,,,)
      (format-polymers ,,,)
  )
)

(defn clean-polymer
      [polymer-str remove]
      (string/replace polymer-str
                      (re-pattern (string/join [(.toLowerCase (str remove)) "|" (.toUpperCase (str remove))]))
                      ""
      )
)

(defn find-shortest-polymer
  [input]
  (->> input
       (.toUpperCase ,,,)
       (into #{} ,,,)
       (map #(clean-polymer input %) ,,,)
       (reduce #(min %1 (count (alchemical-reduction %2))) Integer/MAX_VALUE ,,,)
  )
)

(defn dayFivePart1 []
      (println "Day 5, part 1: the length of the polymer is:")
      (count (alchemical-reduction (slurp "resources/eighteen/dayFive.txt")))
)

(defn dayFivePart2 []
  (println "Day 5, part 2: The shortest cleaned polymer is:")
  (find-shortest-polymer (slurp "resources/eighteen/dayFive.txt"))
)

; day 6
; given a point [x, y] find closest label point [A]
; if only A is min-distance from [x y]
;   assign [x,y] to A
; else
;   do nothing
; group all points by input label
; remove labels w. unbounded areas
; find the label w. the most points assigned

(defn manhattan
      [[x y] [u v]]
      (+ (Math/abs (- u x)) (Math/abs (- v y)))
)

(defn nearest-label
  [p labels]
      ;(reduce #(assoc %1 %2 (manhattan p %2)) {} labels)
      (->> labels
           (group-by #(manhattan p %) ,,,)
           (sort ,,,)
           (first ,,,)
           (second ,,,)
      )
)