//
//  MenuView.swift
//  Libellule
//
//  Created by Jules on 15/08/2026.
//

import SwiftUI
import LibelluleKit

struct MenuView: View {
    @State var store: TimetableStore
    @State private var selectedMeal: Meal = .lunch
    
    var body: some View {
        LoadableView(state: store.menu, retry: { await store.loadMenu() }) { menu in
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
                        if let starter = lunch.starter {
                            Text("Entrée").font(.headline)
                            ForEach(starter.food, id: \.id) { food in
                                Text(food.label)
                            }
                        }
                        if let main = lunch.main {
                            Text("Plat").font(.headline)
                            ForEach(main.food, id: \.id) { food in
                                Text(food.label)
                            }
                        }
                        if let trimmings = lunch.trimmings {
                            Text("Accompagnement").font(.headline)
                            ForEach(trimmings.food, id: \.id) { food in
                                Text(food.label)
                            }
                        }
                        if let dairies = lunch.dairies {
                            Text("Produits laitiers").font(.headline)
                            ForEach(dairies.food, id: \.id) { food in
                                Text(food.label)
                            }
                        }
                        if let desserts = lunch.desserts {
                            Text("Dessert").font(.headline)
                            ForEach(desserts.food, id: \.id) { food in
                                Text(food.label)
                            }
                        }
                    } else {
                        ContentUnavailableView("Pas de menu", systemImage: "menucard", description: Text("Le menu n'a été renseigné pour cette date"))
                    }
                case .dinner:
                    if let dinner = menu.dinner {
                        if let starter = dinner.starter {
                            Text("Entrée").font(.headline)
                            ForEach(starter.food, id: \.id) { food in
                                Text(food.label)
                            }
                        }
                        if let main = dinner.main {
                            Text("Plat").font(.headline)
                            ForEach(main.food, id: \.id) { food in
                                Text(food.label)
                            }
                        }
                        if let trimmings = dinner.trimmings {
                            Text("Accompagnement").font(.headline)
                            ForEach(trimmings.food, id: \.id) { food in
                                Text(food.label)
                            }
                        }
                        if let dairies = dinner.dairies {
                            Text("Produits laitiers").font(.headline)
                            ForEach(dairies.food, id: \.id) { food in
                                Text(food.label)
                            }
                        }
                        if let desserts = dinner.desserts {
                            Text("Dessert").font(.headline)
                            ForEach(desserts.food, id: \.id) { food in
                                Text(food.label)
                            }
                        }
                    } else {
                        ContentUnavailableView("Pas de menu", systemImage: "menucard", description: Text("Le menu n'a été renseigné pour cette date"))
                    }
                }
                
                Spacer()
            }
            .padding()
        }
        .task {
            await store.loadMenu()
        }
        .navigationTitle("Menu")
    }
}

enum Meal {
    case lunch, dinner
}
