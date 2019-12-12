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

(defn reverse-y [point]
  (mapv * point [1 -1]))

(defn transform [point origo]
  (mapv - (reverse-y point) origo))

(defn with-pos
  ([y row]
   (map-indexed #(vector [%1 y] %2) row))
  ([y row translation]
   (map-indexed #(vector (transform [%1 y] translation) %2) row)))

(defn asteroid-points
  ([asteroid-map]
   (->> asteroid-map
        (map-indexed #(with-pos %1 %2) ,,,)
        (reduce concat ,,,)
        (filter #(= \# (last %))) ,,,))
  ([translation asteroid-map]
   (->> asteroid-map
        (map-indexed #(with-pos %1 %2 translation) ,,,)
        (reduce concat ,,,)
        (filter #(= \# (last %))) ,,,)))

(defn vector-norm [v]
  (->> v
   (map #(* % %) ,,,)
   (reduce + ,,,)
   (Math/sqrt ,,,)))

(defn direction-vec [p1 p2]
  (let [v (map - p2 p1)
        norm (bigdec (vector-norm v))]
    (mapv #(if (zero? norm) 0 (with-precision 3 (/ % norm))) v))
  )

(defn count-visible-asteroids
  [p points]
  (->> points
       (map #(direction-vec p %) ,,,)
       (distinct ,,,)
       (filter #(not (= '(0 0) %)) ,,,)
       (count ,,,))
  )

(defn find-optimal-asteroid
  [points]
  (->> points
       (map #(hash-map (count-visible-asteroids % points) %) ,,,)
       (apply merge ,,,)
       (apply max-key key ,,,)
       )
  )

(defn asteroid-count
  [asteroid-str]
  (->> asteroid-str
       (string/split-lines)
       (mapv vec)
       (asteroid-points)
       (map first)
       (find-optimal-asteroid)
       ))

(defn day-ten-part-one []
  (println "Optimal asteroid is:")
  (asteroid-count (slurp "resources/nineteen/dayTen.txt")))

; Day 10, part 2: Shift origo to selected asteroid.
; shift by changing sign of y-values.
; then subtracting px from all x values.
; then subtracting py from all y values.
; sort visible asteroids by angle to y-axis.
; caclulate theta, map all negative angles to 
; pi/2 - theta
; origo = [8 3]

(defn phi [[x y]]
  "The angle between a point and the y-axis. 0 <= phi < 2pi"
  (let [phi (- (/ Math/PI 2) (Math/atan2 y x))]
    (if (neg? phi) (+ (* 2 Math/PI) phi) phi)))

(defn visible
  "Get set of visible asteroids, sorted according to phi, i.e vaporization order"
  [asteroids]
  (->> (vals asteroids)
       (group-by :phi ,,,)
       (sort ,,,)
       (map #(first (sort-by :r (val %))) ,,,)
       (map :point ,,,))
  )

(defn vaporize
  [asteroids]
  (loop [asteroids asteroids
         vaporized []]
    (if (empty? asteroids)
      vaporized
      (let [to-be-vaporized (visible asteroids)]
        (recur
         (reduce dissoc asteroids to-be-vaporized)
         (concat vaporized to-be-vaporized)))
      )
    )
  )

(defn transform-asteroid-map
  [asteroid-str x y]
  (->> asteroid-str
       (string/split-lines)
       (mapv vec)
       (asteroid-points [x y])
       (map #(hash-map (first %) {:phi (phi (first %))
                                  :r (vector-norm (first %))
                                  :point (first %)}))
       (apply merge ,,,)
       )
  )

(defn score
  [[x y]]
  (+ (* x 100) y)
  )

(defn log [value & msg]
  (do (println value msg) value))

(defn day-ten-part-two []
  (-> "resources/nineteen/dayTen.txt"
      (slurp ,,,)
      (transform-asteroid-map ,,, 22 -19)
      (vaporize ,,,)
      (nth ,,, 199)
      (transform ,,, [-22 -19])
      (log ,,, " is the last asteroid")
      (score ,,,)
      ))