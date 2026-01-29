package Kata;

import com.sun.source.tree.BreakTree;

public class SpinningWords {
    public static String spinWords(String s) {
        String[] a = s.split(" ");
        for (int i = 0; i < a.length; i++) {
            if (a[i].length() >= 5) {
                String[] ai = a[i].split("");
                for (int j = 0; j < ai.length / 2; j++) {
                    String tmp = ai[j];
                    ai[j] = ai[ai.length-j-1];
                    ai[ai.length-j-1] = tmp;
                }
                a[i] = String.join("", ai);
            }
        }
        return String.join(" ", a);
    }
    // or (faster) ((((((((((((
    public static String spinWordss(String s) {
        String[] w = s.split(" ");
        for (int i = 0; i < w.length; i++) {
            if (w[i].length() >= 5)
                w[i] = new StringBuilder(w[i]).reverse().toString();
        }
        return String.join(" ", w);
    }
}
