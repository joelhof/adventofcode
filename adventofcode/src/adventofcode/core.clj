(ns adventofcode.core
    (:require [clojure.string :as string])
    )


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
                                   (map (fn [x] (filter #(= (mod % x) 0) sortedRow)) sortedRow)))
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
             ;"We are at top right corner, turn LEFT"
             (= (first corners) currentPos) [(- 1) 0]
             ;"We are at top left corner, turn DOWN"
             (= (second corners) currentPos) [0 (- 1)]
             ;"We are at bottom left corner, turn RIGHT"
             (= (nth corners 2) currentPos) [1 0]
             ;"We are at first pos in a new layer, move UP
             (= (layerStart (first (spiralLayer n))) currentPos) [0 1]
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

(defn surroundingPositions
      [pos]
      (map #(vec (map + % pos)) [[1 0] [0 1] [-1 0] [0 -1] [-1 -1] [1 1] [1 -1] [-1 1]])
      )

(defn summation
      [positions s]
      (reduce + (map #(s % 0) positions))
      )

(defn s_ij
      [pos s]
      (-> pos
          (surroundingPositions,,,)
          (summation,,, s)
          )
      )

(defn spiralMemoryStress
      ""
      ([n] (spiralMemoryStress n {[0 0] 1} [0 1] [1 0]))
      ([n s previousMove currentPos]
        ;    (println n previousMove currentPos s)
        (if (> (apply max (vals s)) n)
          (s (vec (map - currentPos previousMove)))
          (let [nextMove (getNextMove (+ (count s) 1) previousMove currentPos)]
               (spiralMemoryStress n (assoc s currentPos (s_ij currentPos s))
                                   nextMove (vec (map + nextMove currentPos)))
               )
          )
        )
      )

(defn isPassphraseValid?
      "Only words with distinct letter distributions are allowed, i.e no anagrams"
      [pass]
      (let [words (string/split pass #" ")]
           (if (= (count words) (count (distinct (map frequencies words))))
             1
             0)
           )
      )

(defn escape?
      [position instructions]
      (let [move (nth instructions position)]
           (if (neg? move)
             (> (Math/abs move) position)
             (>= move (- (count instructions) position))
             )
           )
      )

(def counter (atom 0))

(defn getUpdatedInstr [instructions currentPos]
      (if (> (nth instructions currentPos) 2)
        (update instructions currentPos dec)
        (update instructions currentPos inc)
        )
      )

(defn jump
      ([instructions] (jump instructions 0))
      ([instructions currentPos]
        (swap! counter inc)
        (if (escape? currentPos instructions)
          (deref counter)
          (recur (getUpdatedInstr instructions currentPos)
                 (+ currentPos (nth instructions currentPos)))
          )
        )
      )

(defn bankToRedistribute
      [banks]
      (apply max banks)
      )

(defn indexToRedistribute
      [banks index]
      (nth (take
             (+ (count banks) (bankToRedistribute banks))
             (cycle (range (count banks))))
           (inc index))
      )

(defn redistribute
      "Redistribute memory blocks among banks"
      [banks]
      (loop [bank (bankToRedistribute banks)
             index (.indexOf banks bank)
             banks (update banks index (fn [old] 0))]
            (if (zero? bank)
              banks
              (recur
                (dec bank)
                (indexToRedistribute banks index)
                (update banks (indexToRedistribute banks index) inc)
                )
              )
            )
      )

(defn reallocate
      "Reallocate memory blocks until a config is repeated"
      [banks]
      (loop [visited [banks]
             banks (redistribute banks)
             steps 1]
            (if (some #{banks} visited)
              [steps (- (count visited) (.indexOf visited banks))]
              (recur (conj visited banks) (redistribute banks) (inc steps))
              )
            )
      )

(defn parentNode
      [parentStr]
      (let [parent (string/split parentStr #" ")]
           {:name (first parent) :weight (second parent)})
      )

(defn getChildren
      [nodeStr]
      (if (.contains nodeStr "->")
        (mapv string/trim (string/split (second (string/split nodeStr #"->")) #","))
        []
        )
      )

(defn parseNode
      [nodeStr]
      (assoc (parentNode nodeStr) :children (getChildren nodeStr))
      )

(defn findRoot
      [nodeStrings]
      (let [nodes (map parseNode nodeStrings)]
           (loop [parent (nth nodes 0)
                  root nil]
                 (if (nil? parent)
                   root
                   (recur (first (filter #(.contains (set (:children %)) (:name parent)) nodes))
                          parent)
                   )
                 )
           )
      )

(defn intWeight
  [node]
    (Integer/valueOf (re-find #"\d+" (:weight node)))
)

(defn findUnbalancedChild
      [root childWeights nodes]
      (let [weights (sort-by val (frequencies childWeights))
            unbalancedWeight (ffirst weights)
            unbalancedChild (filter #(= (:totalWeight %) unbalancedWeight)
                                    (map #((keyword %) nodes) (:children (root nodes))))
            ]
           (assoc (first unbalancedChild) :diff (reduce - (map first weights)))
      )
)

(defn setTotalWeight!
  [nodes n totalWeight]
      (swap! nodes assoc-in [n :totalWeight] totalWeight)
      (:totalWeight (n @nodes))
)

(defn unbalanced
      [childWeights]
      (filter #(instance? clojure.lang.PersistentArrayMap %) childWeights)
      )

(defn balance
  ""
  [root nodes]
      (if (empty? (:children (root @nodes)))
         (setTotalWeight! nodes root (intWeight (root @nodes)))
         (let [childWeights (reduce conj []
                       (map #(balance (keyword %) nodes) (:children (root @nodes))))]
              (if (empty? (unbalanced childWeights))
                (if (> (count (frequencies childWeights)) 1)
                  (findUnbalancedChild root childWeights @nodes)
                  (setTotalWeight! nodes root (reduce + (intWeight (root @nodes)) childWeights))
                )
                (first (unbalanced childWeights))
              )
         )
      )
)

(defn balanceTree
      [nodeStrings]
      (let [nodeMap (atom (into {}
                                (for [n (map parseNode nodeStrings)]
                                     [(keyword (:name n)) n])))
            root (findRoot nodeStrings)]
           (balance (keyword (:name root)) nodeMap)
      )
)

(def r (atom {}))

(defn registerValue
      [x]
      (list 'get '@r x 0)
      )

(defn negate [expr]
      "expr is list. If first character is !, negate expr, otherwise return expr"
      (if (-> expr
              (first)
              (str)
              (.startsWith "!"))
        (cons 'not (list (cons '= (rest expr))))
        expr
      )
)

(defn toPrefix [expr]
      "Converts expr in the form 'a * b' to Clojure expression (* (get r a 0) b).
      Also maps operand 'inc' to '+' and 'dec' to '-'"
      (list (-> (second expr)
                (string/replace "inc" "+")
                (string/replace  "dec" "-")
                (string/replace  "==" "=")
                (read-string )
                )
            (registerValue (first expr)) (read-string (last expr))
      )
)

(defn parseInstruction [instrString]
      (let [tmp (split-at 3 (string/split instrString #" "))
            condition (second tmp)
            operation (first tmp)]
           (list (read-string (first condition))
                 (negate (toPrefix (rest condition)))
                 (list 'assoc '@r (first operation) (toPrefix operation))
                 )
           )
      )

(defn evaluateInstruction
      [instr]
      ;(println (parseInstruction instr))
      (eval (parseInstruction instr))
)

(defn updateRegister! [register]
      (if (nil? register)
         @r
        (reset! r register)
        )
)

(defn evaluateInstructions
      [instructions]
   (doall (map #(updateRegister! (evaluateInstruction %)) instructions))
)


(defn currentMaxRegisterValue
      [instructions]
      (apply max (map val (last (evaluateInstructions instructions)))
       )
)

(defn allTimeMaxRegisterValue
      [instructions]
      (->> instructions
          (evaluateInstructions)
          (mapcat #(map val %))
          (apply max)
      )

)