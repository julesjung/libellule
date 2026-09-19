//
//  ClientFactory.swift
//  Libellule
//
//  Created by Jules on 19/09/2026.
//

import Foundation
import LibelluleKit

struct ClientFactory {
    static func make() async throws -> Client {
        guard let data = UserDefaults.standard.data(forKey: "app"),
              let parameters = try? JSONDecoder().decode(AppParameters.self, from: data),
              let username = try? KeychainService.shared.load(forKey: "username"),
              let password = try? KeychainService.shared.load(forKey: "password") else {
            throw AppError.notLoggedIn
        }

        let instance = try await Instance(url: parameters.instanceUrl)
        return try await Client(instance: instance, username: username, password: password)
    }
}
