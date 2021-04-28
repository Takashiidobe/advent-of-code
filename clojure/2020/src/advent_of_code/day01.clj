(ns advent-of-code.day01)

(defn- read-numbers [data]
  (->> data
       (re-seq #"-?\d+")
       (map #(Integer/parseInt %))))

(defn part-1
  "Day 01 Part 1"
  [input]
  (reduce
    (fn [a b]
      if (= (+ a b) 2020))
    read-numbers input)
  )

(defn part-2
  "Day 01 Part 2"
  [input]
  input)
