package Kata;

import java.util.*;

public class DupleEncoder {
    public static String encode(String s) {
        s = s.toLowerCase();
        s = s.replace(")", "+");
        s = s.replace("(", "-");
        String[] as = s.split("");
        Map<String, Integer> map = new HashMap<>();

        for (int i = 0; i < s.length(); i++) {
            if (map.containsKey(as[i]))
                map.put(as[i], map.get(as[i])+1);
            else
                map.put(as[i], 1);
        }
        for (Map.Entry<String, Integer> e: map.entrySet()) {
            if (e.getValue() > 1)
                s = s.replace(e.getKey(), ")");
            else
                s = s.replace(e.getKey(), "(");
        }
        return s;
    }
    public static String encode2(String s) {
        s = s.toLowerCase();
        String res = "";
        for (int i = 0; i < s.length(); i++) {
            char c = s.charAt(i);
            res += s.lastIndexOf(c) == s.indexOf(c) ? "(" : ")";
        }
        return res;
    }
}
