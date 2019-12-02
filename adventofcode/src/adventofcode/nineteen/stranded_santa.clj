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
  (assoc data k (+ (nth data i) (nth data j))))

(defn opcode-multiply
  [[i j k] data]
  (assoc data k (* (nth data i) (nth data j))))

(defn int-code-step
  ([input] (int-code-step (take 4 input) input))
  ([instruction data] (case (first instruction)
                        99 data
                        1 (opcode-sum (rest instruction) data)
                        2 (opcode-multiply (rest instruction) data))))

(defn int-code-computer
  [input] (take 4 input))

