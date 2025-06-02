use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct PlayerId(u32);

impl PlayerId {
    pub fn new(id: u32) -> Self {
        PlayerId(id)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct GridPos(u32);

impl GridPos {
    pub fn new(pos: u32) -> Self {
        GridPos(pos)
    }
}

// Represents the status of a grid in storage
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum GridStoreStatus {
    Stored,
    NotStored,
}

// Represents the loading status of a grid
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum GridLoadStatus {
    Loaded,
    NotLoaded,
}

// Represents the dirty status of a loaded grid
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum GridDirtyStatus {
    Dirty,
    Unmodified,
    Pristine,
    Unknown, // Used for initial state of not_loaded grids
}

pub struct GridSystem {
    // Variables (representing the state of the system)
    grid_store_status: HashMap<GridPos, GridStoreStatus>,
    grid_load_status: HashMap<GridPos, GridLoadStatus>,
    grid_dirty_status: HashMap<GridPos, GridDirtyStatus>,
    player_positions: HashMap<PlayerId, GridPos>,

    // Constants (using the new type system, these are sets of values we created)
    all_grid_positions: HashSet<GridPos>,
    all_player_ids: HashSet<PlayerId>,
}

impl GridSystem {
    pub fn new(all_grid_positions: HashSet<GridPos>, all_player_ids: HashSet<PlayerId>) -> Self {
        let mut system = Self {
            grid_store_status: HashMap::new(),
            grid_load_status: HashMap::new(),
            grid_dirty_status: HashMap::new(),
            player_positions: HashMap::new(),
            all_grid_positions,
            all_player_ids,
        };
        system.initialize_system();
        system
    }

    // Initial State
    fn initialize_system(&mut self) {
        for &pos in &self.all_grid_positions {
            self.grid_store_status
                .insert(pos, GridStoreStatus::NotStored);
            self.grid_load_status.insert(pos, GridLoadStatus::NotLoaded);
            self.grid_dirty_status.insert(pos, GridDirtyStatus::Unknown);
        }

        // For player positions, pick an arbitrary initial position
        let initial_player_pos = *self
            .all_grid_positions
            .iter()
            .next()
            .expect("Grid positions cannot be empty for initialization");
        for &player_id in &self.all_player_ids {
            self.player_positions.insert(player_id, initial_player_pos);
        }
    }

    /* -- Actions -- */

    /// Player moves to new position, and there is a loaded grid at that position.
    ///
    /// # Arguments
    /// * `player_id` - The ID of the player moving.
    /// * `new_pos` - The new position the player is moving to.
    ///
    /// # Returns
    /// `true` if the action was successful, `false` otherwise (e.g., preconditions not met).
    pub fn player_moves(&mut self, player_id: PlayerId, new_pos: GridPos) -> bool {
        // Preconditions
        if self.grid_load_status.get(&new_pos) != Some(&GridLoadStatus::Loaded) {
            return false; // Grid at new_pos must be loaded
        }

        // State update
        self.player_positions.insert(player_id, new_pos);
        // UNCHANGED <<grid_store_status, grid_load_status, grid_dirty_status>> is implicitly handled as they are not modified.
        true
    }

    /// A missing grid is loaded and marked pristine.
    ///
    /// # Arguments
    /// * `pos` - The position of the grid to load.
    ///
    /// # Returns
    /// `true` if the action was successful, `false` otherwise.
    pub fn load_missing_grid(&mut self, pos: GridPos) -> bool {
        // Preconditions
        if self.grid_store_status.get(&pos) != Some(&GridStoreStatus::NotStored) {
            return false; // Grid must be "not_stored" (missing)
        }
        if self.grid_load_status.get(&pos) != Some(&GridLoadStatus::NotLoaded) {
            return false; // Grid must be "not_loaded"
        }

        // State update
        self.grid_load_status.insert(pos, GridLoadStatus::Loaded);
        self.grid_dirty_status
            .insert(pos, GridDirtyStatus::Pristine);
        // UNCHANGED <<grid_store_status, player_positions>>
        true
    }

    /// A stored grid is loaded and marked unmodified.
    ///
    /// # Arguments
    /// * `pos` - The position of the grid to load.
    ///
    /// # Returns
    /// `true` if the action was successful, `false` otherwise.
    pub fn load_stored_grid(&mut self, pos: GridPos) -> bool {
        // Preconditions
        if self.grid_store_status.get(&pos) != Some(&GridStoreStatus::Stored) {
            return false; // Grid must be "stored"
        }
        if self.grid_load_status.get(&pos) != Some(&GridLoadStatus::NotLoaded) {
            return false; // Grid must be "not_loaded"
        }

        // State update
        self.grid_load_status.insert(pos, GridLoadStatus::Loaded);
        self.grid_dirty_status
            .insert(pos, GridDirtyStatus::Unmodified);
        // UNCHANGED <<grid_store_status, player_positions>>
        true
    }

    /// A loaded grid which is pristine or unmodified is marked dirty.
    ///
    /// # Arguments
    /// * `pos` - The position of the grid to mark dirty.
    ///
    /// # Returns
    /// `true` if the action was successful, `false` otherwise.
    pub fn mark_dirty(&mut self, pos: GridPos) -> bool {
        // Preconditions
        if self.grid_load_status.get(&pos) != Some(&GridLoadStatus::Loaded) {
            return false; // Grid must be loaded
        }
        let current_dirty_status = self.grid_dirty_status.get(&pos);
        if current_dirty_status != Some(&GridDirtyStatus::Pristine)
            && current_dirty_status != Some(&GridDirtyStatus::Unmodified)
        {
            return false; // Grid must be pristine or unmodified
        }

        // State update
        self.grid_dirty_status.insert(pos, GridDirtyStatus::Dirty);
        // UNCHANGED <<grid_store_status, grid_load_status, player_positions>>
        true
    }

    /// A loaded grid which is dirty or unmodified is stored (persistently).
    ///
    /// # Arguments
    /// * `pos` - The position of the grid to store.
    ///
    /// # Returns
    /// `true` if the action was successful, `false` otherwise.
    pub fn store_grid(&mut self, pos: GridPos) -> bool {
        // Preconditions
        if self.grid_load_status.get(&pos) != Some(&GridLoadStatus::Loaded) {
            return false; // Grid must be loaded
        }
        let current_dirty_status = self.grid_dirty_status.get(&pos);
        if current_dirty_status != Some(&GridDirtyStatus::Dirty)
            && current_dirty_status != Some(&GridDirtyStatus::Unmodified)
        {
            return false; // Grid must be dirty or unmodified
        }

        // State update
        self.grid_store_status.insert(pos, GridStoreStatus::Stored);
        self.grid_dirty_status
            .insert(pos, GridDirtyStatus::Unmodified); // After storing, it's considered unmodified in memory
                                                       // UNCHANGED <<grid_load_status, player_positions>>
        true
    }

    /// A loaded grid which is pristine or unmodified is set "not loaded" (unloaded from memory).
    ///
    /// # Arguments
    /// * `pos` - The position of the grid to unload.
    ///
    /// # Returns
    /// `true` if the action was successful, `false` otherwise.
    pub fn unload_pristine_unmodified_grid(&mut self, pos: GridPos) -> bool {
        // Preconditions
        if self.grid_load_status.get(&pos) != Some(&GridLoadStatus::Loaded) {
            return false; // Grid must be loaded
        }
        let current_dirty_status = self.grid_dirty_status.get(&pos);
        if current_dirty_status != Some(&GridDirtyStatus::Pristine)
            && current_dirty_status != Some(&GridDirtyStatus::Unmodified)
        {
            return false; // Grid must be pristine or unmodified
        }

        // State update
        self.grid_load_status.insert(pos, GridLoadStatus::NotLoaded);
        // UNCHANGED <<grid_store_status, grid_dirty_status, player_positions>>
        true
    }

    /// A loaded grid which is dirty is stored (persistently) and set unmodified (after storing).
    ///
    /// # Arguments
    /// * `pos` - The position of the grid to store and update its dirty status.
    ///
    /// # Returns
    /// `true` if the action was successful, `false` otherwise.
    pub fn store_and_unload_dirty_grid(&mut self, pos: GridPos) -> bool {
        // Preconditions
        if self.grid_load_status.get(&pos) != Some(&GridLoadStatus::Loaded) {
            return false; // Grid must be loaded
        }
        if self.grid_dirty_status.get(&pos) != Some(&GridDirtyStatus::Dirty) {
            return false; // Grid must be dirty
        }

        // State update
        self.grid_store_status.insert(pos, GridStoreStatus::Stored);
        self.grid_dirty_status
            .insert(pos, GridDirtyStatus::Unmodified); // After storing, it's considered unmodified in memory
                                                       // UNCHANGED <<grid_load_status, player_positions>>
                                                       // Note: As in the Java version, the TLA+ action "StoreAndUnloadDirtyGrid"
                                                       // only stores and sets dirty status to unmodified. It does *not* set
                                                       // grid_load_status to "not_loaded". If unloading is desired, it would be
                                                       // a separate step or a combined action that also changes grid_load_status.
        true
    }

    // --- Helper methods for checking state (optional, for debugging/testing) ---
    pub fn get_grid_store_status(&self, pos: GridPos) -> Option<GridStoreStatus> {
        self.grid_store_status.get(&pos).copied()
    }

    pub fn get_grid_load_status(&self, pos: GridPos) -> Option<GridLoadStatus> {
        self.grid_load_status.get(&pos).copied()
    }

    pub fn get_grid_dirty_status(&self, pos: GridPos) -> Option<GridDirtyStatus> {
        self.grid_dirty_status.get(&pos).copied()
    }

    pub fn get_player_position(&self, player_id: PlayerId) -> Option<GridPos> {
        self.player_positions.get(&player_id).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialization() {
        let mut grid_positions = HashSet::new();
        grid_positions.insert(GridPos::new(1));
        grid_positions.insert(GridPos::new(2));
        let mut player_ids = HashSet::new();
        player_ids.insert(PlayerId::new(101));

        let system = GridSystem::new(grid_positions.clone(), player_ids.clone());

        for pos in grid_positions {
            assert_eq!(
                system.get_grid_store_status(pos),
                Some(GridStoreStatus::NotStored)
            );
            assert_eq!(
                system.get_grid_load_status(pos),
                Some(GridLoadStatus::NotLoaded)
            );
            assert_eq!(
                system.get_grid_dirty_status(pos),
                Some(GridDirtyStatus::Unknown)
            );
        }

        for player_id in player_ids {
            assert!(system.get_player_position(player_id).is_some());
        }
    }

    #[test]
    fn test_player_moves() {
        let mut grid_positions = HashSet::from([GridPos::new(1), GridPos::new(2), GridPos::new(3)]);
        let mut player_ids = HashSet::from([PlayerId::new(101)]);
        let mut system = GridSystem::new(grid_positions, player_ids);

        // Player can't move to an unloaded grid
        assert!(!system.player_moves(PlayerId::new(101), GridPos::new(2)));

        // Load grid 2
        assert!(system.load_missing_grid(GridPos::new(2)));
        assert_eq!(
            system.get_grid_load_status(GridPos::new(2)),
            Some(GridLoadStatus::Loaded)
        );

        // Player can now move to grid 2
        assert!(system.player_moves(PlayerId::new(101), GridPos::new(2)));
        assert_eq!(
            system.get_player_position(PlayerId::new(101)),
            Some(GridPos::new(2))
        );
    }

    #[test]
    fn test_load_missing_grid() {
        let mut grid_positions = HashSet::from([1]);
        let player_ids = HashSet::from([101]);
        let mut system = GridSystem::new(grid_positions.clone(), player_ids.clone());

        // Grid is initially not_stored and not_loaded
        assert_eq!(
            system.get_grid_store_status(1),
            Some(GridStoreStatus::NotStored)
        );
        assert_eq!(
            system.get_grid_load_status(1),
            Some(GridLoadStatus::NotLoaded)
        );
        assert_eq!(
            system.get_grid_dirty_status(1),
            Some(GridDirtyStatus::Unknown)
        );

        assert!(system.load_missing_grid(1));

        assert_eq!(
            system.get_grid_store_status(1),
            Some(GridStoreStatus::NotStored)
        ); // Unchanged
        assert_eq!(system.get_grid_load_status(1), Some(GridLoadStatus::Loaded));
        assert_eq!(
            system.get_grid_dirty_status(1),
            Some(GridDirtyStatus::Pristine)
        );
    }

    #[test]
    fn test_load_stored_grid() {
        let mut grid_positions = HashSet::from([1]);
        let player_ids = HashSet::from([101]);
        let mut system = GridSystem::new(grid_positions.clone(), player_ids.clone());

        // Manually set a grid to stored and not loaded for testing this action
        system.grid_store_status.insert(1, GridStoreStatus::Stored);
        system.grid_load_status.insert(1, GridLoadStatus::NotLoaded);
        system.grid_dirty_status.insert(1, GridDirtyStatus::Unknown);

        assert_eq!(
            system.get_grid_store_status(1),
            Some(GridStoreStatus::Stored)
        );
        assert_eq!(
            system.get_grid_load_status(1),
            Some(GridLoadStatus::NotLoaded)
        );

        assert!(system.load_stored_grid(1));

        assert_eq!(
            system.get_grid_store_status(1),
            Some(GridStoreStatus::Stored)
        ); // Unchanged
        assert_eq!(system.get_grid_load_status(1), Some(GridLoadStatus::Loaded));
        assert_eq!(
            system.get_grid_dirty_status(1),
            Some(GridDirtyStatus::Unmodified)
        );
    }

    #[test]
    fn test_mark_dirty() {
        let mut grid_positions = HashSet::from([1]);
        let player_ids = HashSet::from([101]);
        let mut system = GridSystem::new(grid_positions.clone(), player_ids.clone());

        // Load a pristine grid
        assert!(system.load_missing_grid(1));
        assert_eq!(
            system.get_grid_dirty_status(1),
            Some(GridDirtyStatus::Pristine)
        );

        assert!(system.mark_dirty(1));
        assert_eq!(
            system.get_grid_dirty_status(1),
            Some(GridDirtyStatus::Dirty)
        );

        // Load an unmodified grid (simulate by loading a stored grid)
        system.grid_load_status.insert(1, GridLoadStatus::NotLoaded);
        system.grid_store_status.insert(1, GridStoreStatus::Stored);
        assert!(system.load_stored_grid(1));
        assert_eq!(
            system.get_grid_dirty_status(1),
            Some(GridDirtyStatus::Unmodified)
        );

        assert!(system.mark_dirty(1));
        assert_eq!(
            system.get_grid_dirty_status(1),
            Some(GridDirtyStatus::Dirty)
        );
    }

    #[test]
    fn test_store_grid() {
        let mut grid_positions = HashSet::from([1]);
        let player_ids = HashSet::from([101]);
        let mut system = GridSystem::new(grid_positions.clone(), player_ids.clone());

        // Load a dirty grid
        assert!(system.load_missing_grid(1));
        assert!(system.mark_dirty(1));
        assert_eq!(system.get_grid_load_status(1), Some(GridLoadStatus::Loaded));
        assert_eq!(
            system.get_grid_dirty_status(1),
            Some(GridDirtyStatus::Dirty)
        );
        assert_eq!(
            system.get_grid_store_status(1),
            Some(GridStoreStatus::NotStored)
        );

        assert!(system.store_grid(1));
        assert_eq!(system.get_grid_load_status(1), Some(GridLoadStatus::Loaded)); // Unchanged
        assert_eq!(
            system.get_grid_dirty_status(1),
            Some(GridDirtyStatus::Unmodified)
        );
        assert_eq!(
            system.get_grid_store_status(1),
            Some(GridStoreStatus::Stored)
        );

        // Test with unmodified grid
        system
            .grid_store_status
            .insert(1, GridStoreStatus::NotStored); // Reset for next test
        system
            .grid_dirty_status
            .insert(1, GridDirtyStatus::Unmodified);

        assert!(system.store_grid(1));
        assert_eq!(
            system.get_grid_store_status(1),
            Some(GridStoreStatus::Stored)
        );
        assert_eq!(
            system.get_grid_dirty_status(1),
            Some(GridDirtyStatus::Unmodified)
        ); // Still unmodified
    }

    #[test]
    fn test_unload_pristine_unmodified_grid() {
        let mut grid_positions = HashSet::from([1]);
        let player_ids = HashSet::from([101]);
        let mut system = GridSystem::new(grid_positions.clone(), player_ids.clone());

        // Load a pristine grid
        assert!(system.load_missing_grid(1));
        assert_eq!(system.get_grid_load_status(1), Some(GridLoadStatus::Loaded));
        assert_eq!(
            system.get_grid_dirty_status(1),
            Some(GridDirtyStatus::Pristine)
        );

        assert!(system.unload_pristine_unmodified_grid(1));
        assert_eq!(
            system.get_grid_load_status(1),
            Some(GridLoadStatus::NotLoaded)
        );
        assert_eq!(
            system.get_grid_dirty_status(1),
            Some(GridDirtyStatus::Pristine)
        ); // Unchanged

        // Load an unmodified grid
        system.grid_load_status.insert(1, GridLoadStatus::NotLoaded);
        system.grid_store_status.insert(1, GridStoreStatus::Stored);
        assert!(system.load_stored_grid(1));
        assert_eq!(system.get_grid_load_status(1), Some(GridLoadStatus::Loaded));
        assert_eq!(
            system.get_grid_dirty_status(1),
            Some(GridDirtyStatus::Unmodified)
        );

        assert!(system.unload_pristine_unmodified_grid(1));
        assert_eq!(
            system.get_grid_load_status(1),
            Some(GridLoadStatus::NotLoaded)
        );
        assert_eq!(
            system.get_grid_dirty_status(1),
            Some(GridDirtyStatus::Unmodified)
        ); // Unchanged
    }

    #[test]
    fn test_store_and_unload_dirty_grid() {
        let mut grid_positions = HashSet::from([1]);
        let player_ids = HashSet::from([101]);
        let mut system = GridSystem::new(grid_positions.clone(), player_ids.clone());

        // Load a dirty grid
        assert!(system.load_missing_grid(1));
        assert!(system.mark_dirty(1));
        assert_eq!(system.get_grid_load_status(1), Some(GridLoadStatus::Loaded));
        assert_eq!(
            system.get_grid_dirty_status(1),
            Some(GridDirtyStatus::Dirty)
        );
        assert_eq!(
            system.get_grid_store_status(1),
            Some(GridStoreStatus::NotStored)
        );

        assert!(system.store_and_unload_dirty_grid(1));
        assert_eq!(system.get_grid_load_status(1), Some(GridLoadStatus::Loaded)); // IMPORTANT: Still loaded as per TLA+
        assert_eq!(
            system.get_grid_dirty_status(1),
            Some(GridDirtyStatus::Unmodified)
        );
        assert_eq!(
            system.get_grid_store_status(1),
            Some(GridStoreStatus::Stored)
        );
    }
}
