package Kata;

import java.util.Arrays;

public class CamelCase {
    public static String toCamelCase(String s) {
        String[] ss = s.split("");
        for (int i = 0; i < ss.length-1; i++)
            if (ss[i].equals("-") || ss[i].equals("_")) {
                ss[i] = " ";
                ss[i+1] = ss[i+1].toUpperCase();
            }

        return String.join("", ss).replace(" ", "");
    }
    public static String toCamelCase2(String s) {
        String[] a = s.split("[-_]");
        return Arrays.stream(a, 1, a.length)
                .map(e -> e.substring(0,1).toUpperCase() + s.substring(1))
                .reduce(a[0], String::concat);
    }
}
