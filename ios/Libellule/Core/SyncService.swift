//
//  SyncService.swift
//  Libellule
//
//  Created by Jules on 24/08/2026.
//

import SwiftUI
import SwiftData
import LibelluleKit

private struct SyncServiceKey: EnvironmentKey {
    static var defaultValue: SyncService? = nil
}

extension EnvironmentValues {
    var syncService: SyncService? {
        get { self[SyncServiceKey.self] }
        set { self[SyncServiceKey.self] = newValue }
    }
}

actor SyncService: ModelActor {
    nonisolated let modelExecutor: any ModelExecutor
    nonisolated let modelContainer: ModelContainer
    
    private var clientTask: Task<Client, Error>?
    
    init(modelContainer: ModelContainer) {
        let modelContext = ModelContext(modelContainer)
        self.modelExecutor = DefaultSerialModelExecutor(modelContext: modelContext)
        self.modelContainer = modelContainer
    }
    
    func refreshSession() throws {
        guard let data = UserDefaults.standard.data(forKey: "app"),
              let parameters = try? JSONDecoder().decode(AppParameters.self, from: data),
              let username = try? KeychainService.shared.load(forKey: "username"),
              let password = try? KeychainService.shared.load(forKey: "password") else {
            throw AppError.notLoggedIn
        }
        
        clientTask = Task {
            let instance = try await Instance(url: parameters.instanceUrl)
            return try await Client(instance: instance, username: username, password: password)
        }
    }
    
    func login(url: String, username: String, password: String) async throws(AppError) -> AppParameters {
        clientTask = Task {
            do {
                let instance = try await Instance(url: url)
                return try await Client(instance: instance, username: username, password: password)
            } catch {
                throw error
            }
        }
        
        do {
            let client = try await client()
            
            let periods = client.periods().map { StoredPeriod(id: $0.id, name: $0.name) }
            let defaultPeriodId = client.defaultPeriod()
            let boundaryDates = client.boundaryDates()
            
            return AppParameters(instanceUrl: url, periods: periods, defaultPeriodId: defaultPeriodId, startDate: boundaryDates.start, endDate: boundaryDates.end)
        } catch {
            throw AppError.invalidCredentials
        }
    }
    
    private func client() async throws -> Client {
        if let task = clientTask {
            return try await task.value
        } else {
            try refreshSession()
            
            return try await clientTask!.value
        }
    }
    
    func refreshDayIfStale(_ date: String) async {
        if let existing = try? modelContext.fetch(
            FetchDescriptor<CachedDay>(predicate: #Predicate { $0.date == date })
        ),
           let fetchedAt = existing.first?.fetchedAt,
           Date.now.timeIntervalSince(fetchedAt) < 15 * 60 {
            return
        }
        await refreshDay(date)
    }
    
    func refreshDay(_ date: String) async {
        guard let client = try? await self.client(),
              let timetable = try? await client.timetable(date: date) else { return }
        
        
        if let existing = try? modelContext.fetch(
            FetchDescriptor<CachedDay>(predicate: #Predicate { $0.date == date })
        ).first {
            modelContext.delete(existing)
        }
        
        modelContext.insert(CachedDay(
            date: date,
            lessons: timetable.lessons.map(CachedLesson.init))
        )
        
        try? modelContext.save()
    }
}
