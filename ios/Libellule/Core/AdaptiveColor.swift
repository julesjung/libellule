//
//  Color.swift
//  Libellule
//
//  Created by Jules on 14/08/2026.
//

import SwiftUI

func interpolate(a: Double, b: Double, t: Double) -> Double {
    return a + t * (b - a)
}

extension Color {
    init(
        hex: String,
        lightBrightness: Double = 0.9,
        darkBrightness: Double = 0.65
    ) {
        let value = Int(hex.dropFirst(), radix: 16) ?? 0

        let r = Double((value >> 16) & 0xFF) / 255
        let g = Double((value >> 8) & 0xFF) / 255
        let b = Double(value & 0xFF) / 255

        let max = Swift.max(r, g, b)
        let min = Swift.min(r, g, b)
        let delta = max - min

        var hue = 0.0

        if delta > 0 {
            if max == r {
                hue = (g - b) / delta
            } else if max == g {
                hue = (b - r) / delta + 2
            } else {
                hue = (r - g) / delta + 4
            }

            hue /= 6

            if hue < 0 {
                hue += 1
            }
        }

        let saturation = max == 0 ? 0 : delta / max

        self = Color(
            hue: hue,
            saturation: interpolate(a: 0, b: 0.5, t: saturation),
            brightness: UITraitCollection.current.userInterfaceStyle == .dark
                ? darkBrightness
                : lightBrightness
        )
    }
}

extension Color {
    init(
        hex: String,
    ) {
        let value = Int(hex.dropFirst(), radix: 16) ?? 0

        let r = Double((value >> 16) & 0xFF) / 255
        let g = Double((value >> 8) & 0xFF) / 255
        let b = Double(value & 0xFF) / 255

        self = Color(red: r, green: g, blue: b)
    }
}

