(ns adventofcode.core)

(defn reverseCaptcha
  "Compare with next digit or
  Compare with digit n steps ahead"
  ([coll] (reverseCaptcha coll 1))
  ([coll, n]
  (reduce +
    (map-indexed
      (fn [index a] (let [b (nth (take (* 2 (count coll)) (cycle coll)) (+ index n))]
                      (if (= a b) a 0)))
      coll)
  ))  
)

(def input
  "111831362354551173134957758417849716877188716338227121869992652972154651632296676464285261171625892888598738721925357479249486886375279741651224686642647267979445939836673253446489428761486828844713816198414852769942459766921928735591892723619845983117283575762694758223956262583556675379533479458964152461973321432768858165818549484229241869657725166769662249574889435227698271439423511175653875622976121749344756734658248245212273242115488961818719828258936653236351924292251821352389471971641957941593141159982696396228218461855752555358856127582128823657548151545741663495182446281491763249374581774426225822474112338745629194213976328762985884127324443984163571711941113986826168921187567861288268744663142867866165546795621466134333541274633769865956692539151971953651886381195877638919355216642731848659649263217258599456646635412623461138792945854536154976732167439355548965778313264824237176152196614333748919711422188148687299757751955297978137561935963366682742334867854892581388263132968999722366495346854828316842352829827989419393594846893842746149235681921951476132585199265366836257322121681471877187847219712325933714149151568922456111149524629995933156924418468567649494728828858254296824372929211977446729691143995333874752448315632185286348657293395339475256796591968717487615896959976413637422536563273537972841783386358764761364989261322293887361558128521915542454126546182855197637753115352541578972298715522386683914777967729562229395936593272269661295295223113186683594678533511783187422193626234573849881185849626389774394351115527451886962844431947188429195191724662982411619815811652741733744864411666766133951954595344837179635668177845937578575117168875754181523584442699384167111317875138179567939174589917894597492816476662186746837552978671142265114426813792549412632291424594239391853358914643327549192165466628737614581458189732579814919468795493415762517372227862614224911844744711698557324454211123571327224554259626961741919243229688684838813912553397698937237114287944446722919198743189848428399356842626198635297851274879128322358195585284984366515428245928111112613638341345371"
  )

(def dayOne
  (reverseCaptcha (mapv #(Integer/parseInt (str %)) (seq input)))
  )

(def dayOne2
  (reverseCaptcha (mapv #(Integer/parseInt (str %)) (seq input)) (/ (count input) 2))
)

(defn rowCheckSum
  [row]
  (- (apply max row) (apply min row))
)

(defn spreadSheetCheckSum
  [f spreadSheet]
  (reduce + (map f spreadSheet))  
)

(defn rowModuloCheckSum
  "Pick out the 2 evenly divisible numbers and divide them"
  [row]
  (let [sortedRow (reverse (sort row))]
   (apply / (first (filter #(= (count %) 2)
                           (map (fn[x] (filter #(= (mod % x) 0) sortedRow)) sortedRow)))
  ))
)

(defn spiralLayer
 "Calculate which spiral layer n, x belongs to"
  [x]
  (let [layers (range 1 (+ (Math/sqrt x) 2) 2)]
    (vector (- (count layers) 1) (last layers))
  )
)

(defn layerCorners
  "Return a vector containing the 4 corners of spiral layer r"
  [r]
    (vector [r r] [(- r) r] [(- r) (- r)] [r (- r)])
)

(defn layerStart
  "Return the coordinates of the first number in a layer
 [x y]"
  [r]
  (let [lastCorner (last (layerCorners r))]
    (vector (lastCorner 0) (+ (lastCorner 1) 1))
  )
)
 
(defn layerStartNumber
  [n]
  (int (+ (Math/pow (- n 2) 2) 1))
)

(defn getNextMove
  "Get the move in a counter clockwise spiral"
  [n previousMove currentPos]
  (let [corners (layerCorners (first (spiralLayer n)))]
     (cond
        ;"We are at top right corner, turn left"
        (= (first corners) currentPos) [(- 1) 0]
        ;"We are at top left corner, turn down"
        (= (second corners) currentPos) [0 (- 1)]
        ;"We are at bottom left corner, turn right"
        (= (nth corners 2) currentPos) [1 0]
        ;"We dont need to turn, just continue along previous axis"
        :else previousMove 
      )
    )
)  
  
(defn spiralStateMachine
  "Return the cartesian coordinates [x,y] of number n located in a spiral matrix"
  ([n] (spiralStateMachine n
         (layerStartNumber (nth (spiralLayer n) 1))
         [0 1]
         (layerStart (first (spiralLayer n)))))
  ([n i previousMove currentPos]
;    (println n i previousMove currentPos)
    (if (= n i) currentPos
      (let [nextMove (getNextMove n previousMove currentPos)]
        (spiralStateMachine n (inc i) nextMove
                                    (vec (map + nextMove currentPos))))
    )
  )
)

(defn manhattanDistance
  "Return the manhattan distance of cartesian coordinates [x,y] of point p"  
  [p]
  (reduce + (map #(Math/abs %) p))
)

(defn spiralMemory
  ""
  [n]
  (manhattanDistance (spiralStateMachine n))
)

