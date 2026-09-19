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
    @Entry var syncService: SyncService = .shared
}

@ModelActor
actor SyncService {
    private var client: Client?
    private var building: Task<Client, any Error>?
    
    static let shared = SyncService(modelContainer: Store.container)
    
    private func currentClient() async throws -> Client {
        if let client { return client }
        if let building { return try await building.value }
        
        let task = Task { try await ClientFactory.make() }
        building = task
        defer { building = nil }
        let fresh = try await task.value
        client = fresh
        
        return fresh
    }
    
    private func withClient<T: Sendable>(_ call: (Client) async throws -> T) async throws -> T {
        do {
            return try await call(currentClient())
        } catch {
            client = nil
            
            return try await call(currentClient())
        }
    }
    
    func login(url: String, username: String, password: String) async throws(AppError) -> AppParameters {
        building = Task {
            do {
                let instance = try await Instance(url: url)
                return try await Client(instance: instance, username: username, password: password)
            } catch {
                throw error
            }
        }
        
        do {
            let parameters = try await withClient { client in
                let periods = client.periods().map { StoredPeriod(id: $0.id, name: $0.name) }
                let defaultPeriodId = client.defaultPeriod()
                let boundaryDates = client.boundaryDates()
                
                return AppParameters(instanceUrl: url, periods: periods, defaultPeriodId: defaultPeriodId, startDate: boundaryDates.start, endDate: boundaryDates.end)
            }
            
            return parameters
        } catch {
            throw AppError.invalidCredentials
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
        guard let timetable = try? await withClient({ client in
            return try? await client.timetable(date: date)
        }) else { return }
        
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
    
    func refreshHomeworkIfStale(_ date: Date) async {
        if let existing = try? modelContext.fetch(
            FetchDescriptor<CachedHomework>(predicate: #Predicate { $0.date == date })
        ),
           let fetchedAt = existing.first?.fetchedAt,
           Date.now.timeIntervalSince(fetchedAt) < 15 * 60 {
            return
        }
        await refreshHomework(date)
    }
    
    func refreshHomework(_ date: Date) async {
        guard let homework = try? await withClient({ client in
            try await client.homework(date: FFIDate.date.string(from: date))
        }) else { return }
        
        if let existing = try? modelContext.fetch(
            FetchDescriptor<CachedHomework>(predicate: #Predicate { $0.date == date })
        ).first {
            modelContext.delete(existing)
        }
        
        modelContext.insert(CachedHomework(date: date, homework: homework))
        
        try? modelContext.save()
    }
    
    func refreshMenuIfStale(_ date: Date) async {
        if let existing = try? modelContext.fetch(
            FetchDescriptor<CachedMenu>(predicate: #Predicate { $0.date == date })
        ),
           let fetchedAt = existing.first?.fetchedAt,
           Date.now.timeIntervalSince(fetchedAt) < 15 * 60 {
            return
        }
        await refreshMenu(date)
    }
    
    func refreshMenu(_ date: Date) async {
        guard let menu = try? await withClient({ client in
            return try await client.menu(date: FFIDate.date.string(from: date))
        }) else { return }
        
        if let existing = try? modelContext.fetch(
            FetchDescriptor<CachedMenu>(predicate: #Predicate { $0.date == date })
        ).first {
            modelContext.delete(existing)
        }
        
        modelContext.insert(CachedMenu(date: date, menu: menu))
        
        try? modelContext.save()
    }
}

