//
//  ClientFactory.swift
//  Libellule
//
//  Created by Jules on 31/08/2026.
//

import Foundation
import LibelluleKit

struct ClientFactory: Sendable {
    func make() async throws -> Client {
        guard let url = UserDefaults.standard.string(forKey: "url"),
              let username = try KeychainService.shared.load(forKey: "username"),
              let password = try KeychainService.shared.load(forKey: "password")
                else { throw AppError.notLoggedIn }
        let instance = try await Instance(url: url)
        let client = try await Client(instance: instance, username: username, password: password)
        
        return client
    }
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
