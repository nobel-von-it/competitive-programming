package Kata;

import java.util.HashSet;
import java.util.List;



public class Isogram {
    public static boolean isIsogram(String s) {
        return s.isEmpty() || s.length() == new HashSet<>(List.of(s.toLowerCase().split(""))).size();
    }
}
