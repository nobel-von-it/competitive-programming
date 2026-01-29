package Kata;

import java.util.Arrays;

public class JadenCase {
    public static String toJadenCase(String s) {
        return (s != null && !s.isEmpty()) ? String.join(" ", Arrays.stream(s.split(" "))
                .map(e -> e.substring(0,1).toUpperCase() + e.substring(1))
                .toArray(String[]::new)) : null;
    }
    public static String toJadenCaseNormal(String s) {
        if (s == null || s.isEmpty()) return null;
        String[] a = s.split(" ");

        for (String string : a) {
            string.split("")[0] = string.split("")[0].toUpperCase();
        }
        return String.join(" ", a);
    }
    public static String toJadenCaseFast(String s) {
        if (s == null || s.isEmpty()) return null;
        char[] a = s.toCharArray();
        for (int i = 0; i < a.length; i++) {
            if (i == 0 || a[i-1] == ' ')
                a[i] = Character.toUpperCase(a[i]);
        }
        return new String(a);
    }
}
