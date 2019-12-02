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
