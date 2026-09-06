//
//  DateKeys.swift
//  Libellule
//
//  Created by Jules on 31/08/2026.
//

import Foundation

enum DateKeys {
    static func today() -> String {
        DateFormatter.date.string(from: .now)
    }
}
