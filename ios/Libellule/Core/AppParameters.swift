//
//  AppParameters.swift
//  Libellule
//
//  Created by Jules on 01/09/2026.
//

import SwiftUI

private struct AppParametersKey: EnvironmentKey {
    static var defaultValue: AppParameters?
}

extension EnvironmentValues {
    var appParameters: AppParameters? {
        get { self[AppParametersKey.self] }
        set { self[AppParametersKey.self] = newValue }
    }
}

nonisolated struct AppParameters: Codable {
    var instanceUrl: String
    var periods: [StoredPeriod]
    var defaultPeriodId: String
    var startDate: String
    var endDate: String
}

nonisolated struct StoredPeriod: Codable {
    var id: String
    var name: String
}

enum AppError: Error {
    case notLoggedIn
    case invalidCredentials
}

extension AppError: LocalizedError {
    var errorDescription: String? {
        switch self {
        case .notLoggedIn:
            return "Please log in"
        case .invalidCredentials:
            return "Invalid credentials"
        }
    }
}
