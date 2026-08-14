//
//  GradesStore.swift
//  Libellule
//
//  Created by Jules on 10/08/2026.
//

import Foundation
import LibelluleKit

@Observable final class GradesStore {
    private let client: Client
    
    var grades: Loadable<GradesData> = .idle
    var periods: [Period]
    var selectedPeriod: Period {
        didSet { if oldValue != selectedPeriod { Task { await loadGrades() } } }
    }
    
    init(client: Client) {
        self.client = client
        
        let periods = client.getPeriods()
        self.periods = periods
        
        let defaultId = client.getDefaultPeriod()
        self.selectedPeriod = periods.first { $0.id == defaultId }!
    }
    
    func loadGrades() async {
        grades = .loading
        do {
            grades = .loaded(try await client.getGrades(period: selectedPeriod))
        } catch {
            grades = .failed(error)
        }
    }
}

enum GradesError: Error {
    case noPeriods
}
