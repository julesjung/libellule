//
//  MenuView.swift
//  Libellule
//
//  Created by Jules on 15/08/2026.
//

import SwiftUI
import SwiftData
import LibelluleKit

struct MenuView: View {
    let date: Date
    @Environment(\.syncService) private var sync
    @Query private var menu: [CachedMenu]
    @State private var selectedMeal: Meal = .lunch
    
    init(date: Date) {
        self.date = date
        self._menu = Query(FetchDescriptor<CachedMenu>(predicate: #Predicate { $0.date == date }))
    }
    
    var body: some View {
        Group {
            if let menu = self.menu.first {
                Picker("Repas", selection: $selectedMeal) {
                    Text("Midi").tag(Meal.lunch)
                    Text("Soir").tag(Meal.dinner)
                }
                .pickerStyle(.segmented)
                .padding()
                
                VStack(alignment: .leading) {
                    switch selectedMeal {
                    case .lunch:
                        if let lunch = menu.lunch {
                            List {
                                if let starter = lunch.starter {
                                    Section("Entrée") {
                                        ForEach(starter.food, id: \.self) { food in
                                            Text(food)
                                        }
                                    }
                                }
                                if let main = lunch.main {
                                    Section("Plat") {
                                        ForEach(main.food, id: \.self) { food in
                                            Text(food)
                                        }
                                    }
                                }
                                if let trimmings = lunch.trimmings {
                                    Section("Accompagnement") {
                                        ForEach(trimmings.food, id: \.self) { food in
                                            Text(food)
                                        }
                                    }
                                }
                                if let dairies = lunch.dairies {
                                    Section("Produits laitiers") {
                                        ForEach(dairies.food, id: \.self) { food in
                                            Text(food)
                                        }
                                    }
                                }
                                if let desserts = lunch.desserts {
                                    Section("Dessert") {
                                        ForEach(desserts.food, id: \.self) { food in
                                            Text(food)
                                        }
                                    }
                                }
                            }
                        } else {
                            ContentUnavailableView("Pas de menu", systemImage: "menucard", description: Text("Le menu n'a été renseigné pour cette date"))
                        }
                    case .dinner:
                        if let dinner = menu.dinner {
                            List {
                                if let starter = dinner.starter {
                                    Section("Entrée") {
                                        ForEach(starter.food, id: \.self) { food in
                                            Text(food)
                                        }
                                    }
                                }
                                if let main = dinner.main {
                                    Section("Plat") {
                                        ForEach(main.food, id: \.self) { food in
                                            Text(food)
                                        }
                                    }
                                }
                                if let trimmings = dinner.trimmings {
                                    Section("Accompagnement") {
                                        ForEach(trimmings.food, id: \.self) { food in
                                            Text(food)
                                        }
                                    }
                                }
                                if let dairies = dinner.dairies {
                                    Section("Produits laitiers") {
                                        ForEach(dairies.food, id: \.self) { food in
                                            Text(food)
                                        }
                                    }
                                }
                                if let desserts = dinner.desserts {
                                    Section("Dessert") {
                                        ForEach(desserts.food, id: \.self) { food in
                                            Text(food)
                                        }
                                    }
                                }
                            }
                        } else {
                            ContentUnavailableView("Pas de menu", systemImage: "menucard", description: Text("Le menu n'a été renseigné pour cette date"))
                        }
                    }
                }
            } else {
                ContentUnavailableView("Pas de menu", systemImage: "menucard", description: Text("Le menu n'a été renseigné pour cette date"))
            }
        }
        .task {
            await sync?.refreshMenu(date)
        }
        .refreshable {
            await sync?.refreshMenuIfStale(date)
        }
        .navigationTitle("Menu")
    }
}

enum Meal {
    case lunch, dinner
}
