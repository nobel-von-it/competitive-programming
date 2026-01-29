package LeetCode;

import java.util.Arrays;

public class MedianOfTwoArrays {
    public static double findMedianSortedArrays(int[] n1, int[] n2) {
        int[] n = Arrays.copyOf(n1, n1.length + n2.length);
        System.arraycopy(n2, 0, n, n1.length, n2.length);
        Arrays.sort(n);
        return n.length % 2 == 0 ? (n[n.length / 2] + n[n.length / 2 - 1]) / 2.0 : n[n.length / 2];
    }
    public static double find(int[] n1, int[] n2) {
        int[] n = new int[n1.length + n2.length];
        System.arraycopy(n1, 0, n, 0, n1.length);
        System.arraycopy(n2, 0, n, n1.length, n2.length);
        System.out.println(Arrays.toString(n));
        Arrays.sort(n);
        int mid = n[n.length / 2], midod = n[n.length / 2 - 1];
        return n.length % 2 == 0 ? (mid + midod) / 2.0 : mid;
    }
    public static double fastFind(int[] n1, int[] n2) {
        if (n1.length > n2.length)
            return fastFind(n2, n1);
        int x = n1.length, y = n2.length;
        int low = 0, high = x;

        while (low <= high) {
            int pX = (low + high) / 2;
            int pY = (x+y+1) / 2 - pX;

            int maxX = (pX == 0) ? Integer.MIN_VALUE : n1[pX - 1];
            int maxY = (pY == 0) ? Integer.MIN_VALUE : n2[pY - 1];
            int minX = (pX == x) ? Integer.MAX_VALUE : n1[pX];
            int minY = (pY == y) ? Integer.MAX_VALUE : n2[pY];

            if (maxX <= minY && maxY <= minX) {
                if ((x + y) % 2 == 0)
                    return (Math.max(maxX, maxY) + Math.min(minX, minY)) / 2.0;
                else return Math.max(maxX, maxY);
            } else if (maxX > minY)
                high = pX - 1;
            else low = pX + 1;
        }
        throw new IllegalArgumentException("error");
    }
}
