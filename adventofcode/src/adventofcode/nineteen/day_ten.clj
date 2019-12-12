(ns adventofcode.nineteen.day-ten
  (:require [clojure.string :as string]))

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
       (find-optimal-asteroid)))
       

(defn part-one []
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

(defn part-two []
  (-> "resources/nineteen/dayTen.txt"
      (slurp ,,,)
      (transform-asteroid-map ,,, 22 -19)
      (vaporize ,,,)
      (nth ,,, 199)
      (transform ,,, [-22 -19])
      (log ,,, " is the last asteroid")
      (score ,,,)))
      
