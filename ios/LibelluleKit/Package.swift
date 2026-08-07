// swift-tools-version: 6.1
import PackageDescription

let package = Package(
    name: "LibelluleKit",
    platforms: [.iOS(.v16)],
    products: [
        .library(name: "LibelluleKit", targets: ["LibelluleKit"])
    ],
    targets: [
        .binaryTarget(
            name: "libelluleFFI",
            path: "./libellule.xcframework"
        ),
        .target(
            name: "LibelluleKit",
            dependencies: [.target(name: "libelluleFFI")],
            path: "Sources/LibelluleKit"
        ),
    ]
)
