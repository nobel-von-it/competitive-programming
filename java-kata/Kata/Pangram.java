package Kata;

import java.util.*;

public class Pangram {
    public static boolean check(String s) {
        String all = "qwertyuiopasdfghjklzxcvbnm";
        int count = 0;
        Set<String> set = new HashSet<>(List.of(s.toLowerCase().split("")));
        System.out.println(all);
        System.out.println(set);
        for (String e: set)
            if (all.contains(e))
                count++;
        return count >= 26;
    }
    //wow
    public static boolean check2(String s) {
        for (char c = 'a'; c <= 'z'; c++)
            if (!s.toLowerCase().contains(String.valueOf(c)))
                return false;
        return true;
    }
}
