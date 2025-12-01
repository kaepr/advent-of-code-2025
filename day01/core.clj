(ns core
  (:require [clojure.string :as str]))

(comment
  (def input (slurp "input.txt")))

(comment input)

(def sample-input
  "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82")

(defn parse-rotation [r]
  [(if (= (first r) \L) - +)
   (read-string (apply str (next r)))])

(defn part-1-rf [{:keys [count dial]} [dir turns]]
  (let [dial (apply dir [dial turns])
        dial (mod (+ 100 dial) 100)]
    {:count (if (= 0 dial)
              (inc count)
              count)
     :dial dial}))

(comment (->> input
              str/split-lines
              (mapv parse-rotation)
              (reduce part-1-rf {:count 0
                                 :dial 50})))

;; There's a nicer mathematical solution possible
;; but this file is too small to contain it
(defn count-zeros [dial turns dir]
  (->> (range dial (apply dir [dial turns]) (if (= dir +) 1 -1))
       (map #(mod % 100))
       (filter zero?)
       count))

(defn part-2-rf [{:keys [count dial]} [dir turns]]
  (let [next-dial (mod (apply dir [dial turns]) 100)]
    {:count (+ count (count-zeros dial turns dir))
     :dial next-dial}))

(comment (->> sample-input
              str/split-lines
              (mapv parse-rotation)
              (reduce part-2-rf {:count 0
                                 :dial 50})))

(comment (->> input
              str/split-lines
              (mapv parse-rotation)
              (reduce part-2-rf {:count 0
                                 :dial 50})))
