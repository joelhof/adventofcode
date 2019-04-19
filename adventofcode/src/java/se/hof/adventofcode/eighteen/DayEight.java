package se.hof.adventofcode.eighteen;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.stream.Collectors;
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
        int[] headers = new int[2];
        List<Node> children;
        String[] metadata;

        private Node(final String[] input) {
            this.headers[0] = Integer.valueOf(input[0]);
            this.headers[1] = Integer.valueOf(input[1]);
            this.metadata = Arrays.copyOfRange(input, input.length - this.headers[1], input.length);
            this.children = new ArrayList<>();
        }

        public int sum() {
            return sum(this);
        }

        private static int sum(Node node) {
            return Arrays.stream(node.metadata)
                    .mapToInt(Integer::valueOf).sum()
                    + node.children.stream()
                    .mapToInt(c -> sum(c)).sum();
        }

        public static Node parse(String s) {
            String[] input = s.split(" ");
            Node node = new Node(input);
            if (node.headers[0] > 0) {
                String[] rest = Arrays.copyOfRange(input, 2, input.length - node.headers[1]);
                node.children.add(parse(String.join(" ", rest)));
            }
            return node;
        }

        private static Node parse(String[] input) {
            if (input.length < 3)
                return null;
            Node node = new Node(input);
            String[] rest = Arrays.copyOfRange(input, 2, input.length - node.headers[1]);
            Node.parse(input);
            return node;
        }
    }
}
