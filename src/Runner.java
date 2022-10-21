import java.io.BufferedReader;
import java.io.FileNotFoundException;
import java.io.FileReader;
import java.io.IOException;
import java.util.*;
import java.util.function.BiConsumer;

public class Runner {
    public static String DIR = "C:\\Users\\xxmem\\Desktop\\school\\4\\Big Data Systems\\A1\\retail.dat";
    public static double SUPPORT_THRESHOLD = 0.01;
    public static double threshold; // As a number of items, not a fraction.
    public static Integer num_baskets;
    public static long startTime;
    public static long end_time = 0;
    public static int feedback_interval = 5000; // Debug: On pass one, how many lines should be parsed before another status update is given
    public static double execution_time; // Debug: updated after each basket
    public static int hashmap_capacity; // To avoid costly resizing on pass2, the program will determine an appropriate size before any insertions.

    public static void main(String[] args) throws IOException {
        startTime = System.currentTimeMillis();
        //Do the first pass, get back a tuple of the counts HashMap, and the number of baskets.
        Tuple<HashMap<Integer, Integer>, Integer> counts = get_item_counts();
        System.out.println("Got item counts.");//Debug

        // Move out of tuple into variables
        HashMap<Integer, Integer> map = counts.get_left();
        num_baskets = counts.get_right();

        threshold = num_baskets * SUPPORT_THRESHOLD; // number of occurances to be considered frequent.

        System.out.println("Calculated threshold.");//Debug

        //Trim the hashmap to exclude items below threshold.
        map.values().removeIf(value -> value < threshold);

        System.out.println("Trimmed hashmap.");//Debug

        //Perform Second Pass, find frequent itemsets
        HashMap<HashSet<Integer>, Integer> ans = find_frequent_pairs(map);

        System.out.println("Found frequent pairs"); //Debug

        System.out.println(ans);

        end_time = System.currentTimeMillis();
        System.out.println("Execution time: " + (end_time - startTime)/1000.0 + "s");

    }

    // Return a tuple of a HashMap relating IDs to their number of occurances on the left, and the total number of baskets on the right.
    public static Tuple<HashMap<Integer, Integer>, Integer> get_item_counts() throws IOException {
        HashMap counts = new HashMap<Integer, Integer>();
        BufferedReader reader;
        int num_rows = 0;
        try {
            reader = new BufferedReader(new FileReader(DIR));
            int baskets_parsed = 0; //For statistics
            String basket;
            basket = reader.readLine();
            while (basket != null) {
                ArrayList<Integer> ints = string_to_int_array(basket);
                num_rows++;
                for (Integer i : ints){
                    if (!counts.containsKey(i)) { //If first time seeing this integer
                        counts.put(i, 1);
                    }
                    else {
                        counts.put(i, (Integer) counts.get(i) + 1); //Increment count by 1.
                    }
                }


                // Progress indicator
                //baskets_parsed++;
                //if ((baskets_parsed % (feedback_interval) == 0)) {
                //    System.out.println("counted basket #" + baskets_parsed);
                //}

                basket = reader.readLine();
            }


        //Boilerplate!
            reader.close();            //////
        } catch (IOException e) {     //////
            e.printStackTrace();     //////
        }
        //Boilerplate!

        return new Tuple(counts, num_rows);
    }


    // Given a string of integers separated by spaces, return those integers as elements in an ArrayList
    public static ArrayList<Integer> string_to_int_array(String basket) {
      //List<String> string_array = Arrays.asList(basket.split(" "));
      ArrayList<String> string_array = new ArrayList();
        int start = 0;
        int end = 0;
        while (true) {
            start = basket.indexOf(" ", end);
            end = basket.indexOf(" ", start + 1);

            if (end == -1){
                break;
            }
            string_array.add(basket.substring(start + 1, end));

        }


        ArrayList<Integer> as_ints = new ArrayList<Integer>();

        for (String s :string_array){
            as_ints.add(Integer.decode(s));
        }

        return as_ints;

    }


    // Return all frequent pairs in the dataset, using a HashMap recording the count of each ID, and a minimum threshold for values to be considered
    public static HashMap<HashSet<Integer>, Integer> find_frequent_pairs(HashMap<Integer, Integer> counts) {
        hashmap_capacity = (int) Math.pow(counts.size(), 2) + 50000; // Possible number of pairs, plus some extra for good measure.
        HashMap<HashSet<Integer>, Integer> frequencies = new HashMap<HashSet<Integer>, Integer>(hashmap_capacity, (float) 1.00); //Only resize if full, should not occur since the map size is over the required capacity.

        BufferedReader reader;
        try {
            reader = new BufferedReader(new FileReader(DIR));
            String basket;
            basket = reader.readLine();
            int baskets_parsed = 0; // To print progress during runtime.
            while (basket != null) {
                ArrayList<Integer> ints = string_to_int_array(basket);


                // For each pair of ints in the basket
                long basket_time_start = System.currentTimeMillis(); //debug
                long basket_time_end; //debug
                for (int i = 0; i <= ints.size() - 2; i++){
                    int candidate1 = ints.get(i);
                    if (counts.containsKey(candidate1)) { //If the element was frequent
                        for (int j = i + 1; j <= ints.size() - 1; j++){
                            int candidate2 = ints.get(j);
                            if (counts.containsKey(candidate2)){
                                //int left = ints.get(i);
                                //int right = ints.get(j);
                                //assert(left < right);
                                //Tuple<Integer, Integer> pair = new Tuple<>(left, right);

                                HashSet<Integer> pair = new HashSet<>(2, (float) 1.0); // Should only contain 2, HashSet's 16 elements is wasteful
                                pair.add(candidate1);
                                pair.add(candidate2);


                                Integer this_pairs_frequency = frequencies.get(pair);
                                if (this_pairs_frequency == null){ // If this is the first time seeing this pair
                                    frequencies.put(pair, 1); // Initialize
                                }
                                else {
                                    if (this_pairs_frequency <= threshold) { // Don't increment if pair is already known as frequent.
                                        frequencies.put(pair, this_pairs_frequency + 1); //Increment count by 1.
                                    }

                                }
                            }

                        }
                    }
                }

                //basket_time_end = System.currentTimeMillis(); //debug
                //execution_time = (basket_time_end - basket_time_start)/1000.0;
                //System.out.println("Execution time: " + (execution_time + "s\tSize of Basket:" + ints.size() + "\tbaskets parsed:" + baskets_parsed)); //debug
                // Progress indicator
                baskets_parsed++;
                if ((baskets_parsed % (num_baskets/100)) == 0) {
                    System.out.println((int)(((double) baskets_parsed / num_baskets) * 100) + "% complete.");

                }

                basket = reader.readLine();
            }


            //Boilerplate!
            reader.close();            //////
        } catch (IOException e) {     //////
            e.printStackTrace();     //////
        }
            //Boilerplate!


        // Trim the final result to exclude below threshold
        frequencies.values().removeIf(value -> value < threshold);

        return frequencies;
    }

}
