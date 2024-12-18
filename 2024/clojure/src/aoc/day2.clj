(ns aoc.day2
  (:require
   [clojure.string :as str]
   [clojure.test :refer [run-tests is deftest]]))

(defonce test-input
  "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9")
(defonce input (slurp "src/aoc/day2.txt"))

(defn split [s] (map read-string (str/split s #" +")))

(defn monotonic? [report] (or (apply < report) (apply > report)))
(defn in-family? [report]
  (every? (fn [pair] (let [[l r] pair
                           abs-diff (abs (- l r))]
                       (and (>= abs-diff 1) (<= abs-diff 3))))
          (partition 2 1 report)))

(defn drop-one-combos [list]
  (for [i (range (count list))]
    (concat (take i list) (drop (inc i) list))))

(defn safe-report? [report] (and (monotonic? report) (in-family? report)))
(defn safe-report-minus-one? [report]
  (reduce #(or %1 %2)
          (map #(and (monotonic? %1) (in-family? %1))
               (drop-one-combos report))))

(deftest safe-report-test
  (is (safe-report? (list 7 6 4 2 1)))
  (is not (safe-report? (list 1 2 7 8 9))))

(defn part1 [data-string]
  (count (filter true? (->> (str/split-lines data-string)
                            (map split)
                            (map safe-report?)))))
(deftest part1-tests (is (= (part1 test-input) 2)))

(defn part2 [data-string]
  (count (filter true? (->> (str/split-lines data-string)
                            (map split)
                            (map safe-report-minus-one?)))))
(deftest part2-tests (is (= (part2 test-input) 4)))

(run-tests)

(println (part1 input))
(println (part2 input))