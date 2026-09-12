//
//  Color.swift
//  Libellule
//
//  Created by Jules on 14/08/2026.
//

import SwiftUI

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

