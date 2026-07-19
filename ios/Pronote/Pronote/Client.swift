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
        self.status = .disconnected
    }
    
    func connect() async throws {
        try await self.client.connect()
        self.status = .connected
    }
    
    func authenticate(username: String, password: String) async throws {
        try await self.client.authenticate(username: username, password: password)
        self.status = .authenticated
    }
    
    func loadUser() async throws {
        try await self.client.loadUser()
        self.status = .ready
    }
    
    func getGrades() async throws -> GradesData {
        let gradesData = try await self.client.getGrades()
        return gradesData
    }
}

enum ClientStatus {
    case disconnected
    case connected
    case authenticated
    case ready
}
