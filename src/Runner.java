import java.io.BufferedReader;
import java.io.FileNotFoundException;
import java.io.FileReader;
import java.io.IOException;
import java.util.*;
import java.util.function.BiConsumer;

public class Runner {
    public static String DIR = "C:\\Users\\xxmem\\Desktop\\school\\4\\Big Data Systems\\A1\\netflix.data";
    public static double SUPPORT_THRESHOLD = 0.01;
    public static double threshold; // As a number of items, not a fraction.
    public static Integer num_baskets;
    public static long end_time = 0;
    public static int feedback_interval = 5000; // On pass one, how many lines should be parsed before another status update is given

    public static void main(String[] args) throws IOException {
        final long startTime = System.currentTimeMillis();
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

        System.out.println(ans.toString());

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
                baskets_parsed++;
                if ((baskets_parsed % (feedback_interval) == 0)) {
                    System.out.println("counted basket #" + baskets_parsed);
                }

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
        List<String> string_array = Arrays.asList(basket.split(" "));
        ArrayList<Integer> as_ints = new ArrayList<Integer>();

        for (String s :string_array){
            as_ints.add(Integer.decode(s));
        }

        return as_ints;

    }


    // Return all frequent pairs in the dataset, using a HashMap recording the count of each ID, and a minimum threshold for values to be considered
    public static HashMap<HashSet<Integer>, Integer> find_frequent_pairs(HashMap<Integer, Integer> counts) {
        HashMap<HashSet<Integer>, Integer> frequencies = new HashMap<>();

        BufferedReader reader;
        try {
            reader = new BufferedReader(new FileReader(DIR));
            String basket;
            basket = reader.readLine();
            int baskets_parsed = 0; // To print progress during runtime.
            while (basket != null) {
                ArrayList<Integer> ints = string_to_int_array(basket);


                // For each pair of ints in the basket
                for (int i = 0; i <= ints.size() - 2; i++){
                    if (counts.containsKey(ints.get(i))) { //If the element was frequent
                        for (int j = i + 1; j <= ints.size() - 1; j++){
                            if (counts.containsKey(ints.get(j))){
                                HashSet<Integer> pair = new HashSet<>();
                                pair.add(ints.get(i));
                                pair.add(ints.get(j));

                                if (!frequencies.containsKey(pair)){ // If this is the first time seeing this pair
                                    frequencies.put(pair, 1); // Initialize
                                }
                                else {
                                    frequencies.put(pair, (Integer) frequencies.get(pair) + 1); //Increment count by 1.
                                }
                            }

                        }
                    }

                }

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
