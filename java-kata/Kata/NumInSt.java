package Kata;

import java.util.Arrays;
import java.util.Objects;

public class NumInSt {
    public static int solve(String s) {
        int res = -1;
        s = s.toLowerCase().replaceAll("[a-z]", " ");
        String[] a = s.split(" ");
        for (String string : a) {
            if (!string.equals(" ") && !string.isEmpty()) {
                res = Math.max(res, Integer.parseInt(string));
            }

        }
        return res;
    }
    public static int solve2(String s) {
        return Arrays.stream(s.split("[a-z]+"))
                .filter(e -> !e.isEmpty())
                .mapToInt(Integer::parseInt)
                .max()
                .getAsInt();
    }
}
