package Kata;
public class Accumul {
    public static String accum(String s) {
        String[] arr = s.toUpperCase().split("");
        for (int i = 0; i < s.length(); i++) {
            for (int j = 1; j <= i; j++) {
                arr[i] += Character.toString(s.charAt(i)).toLowerCase();
            }
        }
        return String.join("-", arr);
    }
}
