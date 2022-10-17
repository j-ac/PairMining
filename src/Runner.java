import java.io.BufferedReader;
import java.io.FileNotFoundException;
import java.io.FileReader;
import java.io.IOException;
import java.util.*;
import java.util.function.BiConsumer;

public class Runner {
    public static String DIR = "/cs/home/jcorr851/Documents/retail.dat";
    public static double SUPPORT_THRESHOLD = 0.01; //
    public static void main(String[] args) throws IOException {
        Tuple<HashMap<Integer, Integer>, Integer> counts = get_item_counts();

        // Separate from tuple
        HashMap<Integer, Integer> map = counts.get_left();
        Integer num_baskets = counts.get_right();

        double threshold = counts.get_right() * SUPPORT_THRESHOLD; // number of occurances to be considered frequent.


        //Trim the hashmap to exclude items below threshold.
        map.values().removeIf(value -> value < threshold);

        System.out.println("Ayy");
        //BRUTE FORCE HERE!

    }

    // Return a tuple of a HashMap relating IDs to their number of occurances on the left, and the total number of baskets on the right.
    public static Tuple<HashMap<Integer, Integer>, Integer> get_item_counts() throws IOException {
        HashMap counts = new HashMap<Integer, Integer>();
        BufferedReader reader;
        int num_rows = 0;
        try {
            reader = new BufferedReader(new FileReader(DIR));
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


    public static ArrayList<Integer> string_to_int_array(String basket) {
        List<String> string_array = Arrays.asList(basket.split(" "));
        ArrayList<Integer> as_ints = new ArrayList<Integer>();

        for (String s :string_array){
            as_ints.add(Integer.decode(s));
        }

        return as_ints;

    }


    // Return all frequent pairs in the dataset, using a HashMap recording the count of each ID, and a minimum threshold for values to be considered
    public int FindFrequentPairs(HashMap<Integer, Integer> counts) {

        return 0;
    }

}
