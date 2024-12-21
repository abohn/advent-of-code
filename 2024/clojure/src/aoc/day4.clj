(ns aoc.day4
  (:require
   [clojure.string :as str]
   [clojure.test :refer [run-tests is deftest]]))

(defonce test-input
  "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX")
(defonce input (slurp "src/aoc/day4.txt"))

(defn rotate [grid] (map str/join (apply map vector grid)))
(defn get-diagonals [grid forward]
  (map str/join (vals
                 (reduce
                  (fn [acc [i row]]
                    (reduce
                     (fn [acc [j val]]
                       (update acc (if forward (- i j) (+ i j)) #(conj (or % []) val)))
                     acc
                     (map-indexed vector row)))
                  {}
                  (map-indexed vector grid)))))

(defn count-xmas-forward [line] (count (re-seq #"XMAS" line)))
(defn count-xmas-line [line]
  (+ (count-xmas-forward line)
     (count-xmas-forward (str/join (reverse line)))))
(defn count-xmas-grid [grid]
  (let [rotated-grid (rotate grid)
        diagonal-right-grid (get-diagonals grid true)
        diagonal-left-grid (get-diagonals grid false)]
    (+ (reduce + (map count-xmas-line grid))
       (reduce + (map count-xmas-line diagonal-right-grid))
       (reduce + (map count-xmas-line rotated-grid))
       (reduce + (map count-xmas-line diagonal-left-grid)))))

(defn part1 [data-string]
  (->> data-string (str/split-lines) (count-xmas-grid)))
(deftest part1-tests (is (= (part1 test-input) 18)))

(def mas-corners #{"MMSS" "MSSM" "SSMM" "SMMS"})
(defn grid-val [grid x y] (nth (nth grid y) x))
(defn is-x-mas? [grid x y]
  (let [center (grid-val grid x y)]
    (if (not= center \A) false
        (let [ul (grid-val grid (- x 1) (- y 1))
              ur (grid-val grid (+ x 1) (- y 1))
              ll (grid-val grid (- x 1) (+ y 1))
              lr (grid-val grid (+ x 1) (+ y 1))]
          (contains? mas-corners (str/join [ul ur lr ll]))))))
(defn count-x-mas [grid]
  (let [height (count grid)
        width (count (first grid))]
    (count (for [x (range 1 (- width 1))
                 y (range 1 (- height 1))
                 :when (is-x-mas? grid x y)]
             [x y]))))

(defn part2 [data-string]
  (->> data-string (str/split-lines) (count-x-mas)))
(part2 test-input)

(deftest part2-tests (is (= (part2 test-input) 9)))

(run-tests)

(println (part1 input))
(println (part2 input))