//
//  Client.swift
//  Pronote
//
//  Created by Jules on 19/07/2026.
//

import SwiftUI
import PronoteKit

@Observable
final class ObservableClient {
    let client: Client
    var status: ClientStatus
    
    init(instanceUrl: String) async throws {
        self.client = try await Client(instanceUrl: instanceUrl)
        self.status = client.status()
    }
    
    func connect() async throws {
        try await self.client.connect()
        self.status = client.status()
    }
    
    func authenticate(username: String, password: String) async throws {
        try await self.client.authenticate(username: username, password: password)
        self.status = client.status()
    }
    
    func userInformation() async throws -> User {
        let user = try await self.client.userInformation()
        self.status = client.status()
        return user
    }
}
