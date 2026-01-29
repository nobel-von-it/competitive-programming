package Kata;

public class Quicksum {
    public static int quicksum(String s) {
        String alp = "ABCDEFGHIJKLMNOPQRSTUVWXYZ ";
        String[] a = s.split("");
        int res = 0;
        for (int i = 0; i < a.length; i++) {
            if (!alp.contains(a[i]) || s.isEmpty())
                return 0;
            if (!a[i].equals(" "))
                res += (i + 1) * (alp.indexOf(a[i]) + 1);
        }
        return res;
    }
}
