package Kata;

import java.util.*;

public class Duplicates {
    public static int duplicateCount(String s) {
        String[] as = s.toLowerCase().split("");
        Map<String, Integer> map = new HashMap<>();
        int res = 0;

        for (int i = 0; i < s.length(); i++) {
            if (map.containsKey(as[i]))
                map.put(as[i], map.get(as[i])+1);
            else
                map.put(as[i], 1);
        }
        for (int i: map.values())
            if (i > 1)
                res++;
        return res;
    }
    public static int duplCount(String s) {
        int res = 0;
        s = s.toLowerCase();
        while (!s.isEmpty()) {
            String f = s.substring(0,1);
            s = s.substring(1);
            if (s.contains(f)) res++;
            s = s.replace(f, "");
        }
        return res;
    }
}
