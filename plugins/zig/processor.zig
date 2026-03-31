const std = @import("std");

pub fn main() !void {
    var stdout = std.io.getStdOut().writer();
    try stdout.print("{{\"tool\":\"zig\",\"status\":\"ok\",\"summary\":\"processor ready\"}}", .{});
}
