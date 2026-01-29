package Kata;

import java.util.Arrays;
import java.util.Objects;

public class WellOfIdeas {
    public static String well(String[] x) {
        int l = Arrays.stream(x).filter(e -> e.equals("good")).toArray().length;
        return l > 2 ? "I smell a series!" : l == 0 ? "Fail!" : "Publish!";
    }
}
