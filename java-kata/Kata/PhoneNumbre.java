package Kata;

import java.util.Arrays;

public class PhoneNumbre {
    public static String createPhoneNumber(int[] n) {
        String s = String.join("", Arrays.toString(n).split(", ")).substring(1, n.length+1);
        return String.format("(%s) %s-%s", s.substring(0, 3), s.substring(3, 6), s.substring(6));
    }
}
