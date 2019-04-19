package se.hof.adventofcode.eighteen;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.stream.Collectors;
import java.util.stream.IntStream;
import java.util.stream.Stream;

/**
 * Try to solve Day 8 with java, for comparision...
 */
public class DayEight {

    public static void main(String[] args) {

        Node n = Node.parse("0 1 99");
        System.out.println("Nr of children: " + n.children.size() + " Expected: 0");
        System.out.println("Metadata: " + n.sum() + " Expected: 99");

    }

    public static class Node {
        public static final int NR_OF_HEADERS = 2;
        int[] headers = new int[2];
        List<Node> children;
        String[] metadata;

        private Node(final String[] input) {
            this.headers[0] = Integer.valueOf(input[0]);
            this.headers[1] = Integer.valueOf(input[1]);
            this.children = new ArrayList<>();
            this.metadata = initMetadata(input);
        }

        private String[] initMetadata(String[] input) {
            return input.length > 2
                    ? Arrays.copyOfRange(input, input.length - this.headers[1], input.length)
                    : new String[headers[1]];
        }

        public int sum() {
            return sum(this);
        }

        private static int sum(Node node) {
            return Arrays.stream(node.metadata)
                    .filter(s -> s != null)
                    .mapToInt(Integer::valueOf).sum()
                    + node.children.stream()
                    .mapToInt(c -> sum(c)).sum();
        }

        public static Node parse(String s) {
            String[] input = s.split(" ");
            Node parent = new Node(input);
            if (parent.headers[0] > 0) {
                int p = NR_OF_HEADERS;
                for (int i = 0; i < parent.headers[0]; i++) {
                    Node child;
                    int to = Integer.valueOf(input[p]) == 0
                            ? p + Integer.valueOf(input[p + 1]) + NR_OF_HEADERS
                            : input.length - parent.headers[1];
                    child = parse(String.join(" ", Arrays.copyOfRange(input, p, to)));
                    p = to;
                    parent.children.add(child);
                }
            }
            return parent;
        }
    }
}
