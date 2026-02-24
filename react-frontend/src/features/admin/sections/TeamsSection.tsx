import { Users } from "lucide-react";
import { useUsers } from "@/hooks/api/useUsers";
import { useTeams } from "@/hooks/api/useTeams";
import { useBusinessUnits } from "@/hooks/api/useBusinessUnits";
import type Team from "@/domain/team/entity";
import type User from "@/domain/user/entity";

function TeamCard({ team, members }: { team: Team; members: User[] }) {
  return (
    <div className="card border border-base-300 bg-base-100">
      <div className="card-body gap-3">
        <h3 className="card-title text-base flex items-center gap-2">
          <Users size={16} />
          {team.name}
        </h3>

        {members.length === 0 ? (
          <p className="text-sm text-base-content/50">No members</p>
        ) : (
          <ul className="space-y-1">
            {members.map((member) => (
              <li key={member.id} className="flex items-center gap-2">
                <div className="avatar placeholder">
                  <div className="bg-neutral text-neutral-content rounded-full w-7">
                    <span className="text-xs">
                      {member.firstName[0]}
                      {member.lastName[0]}
                    </span>
                  </div>
                </div>
                <span className="text-sm">
                  {member.firstName} {member.lastName}
                </span>
              </li>
            ))}
          </ul>
        )}
      </div>
    </div>
  );
}

function UnassignedUsersCard({ users }: { users: User[] }) {
  return (
    <div className="card border border-base-300 bg-base-100">
      <div className="card-body gap-3">
        <h3 className="card-title text-base flex items-center gap-2">
          <Users size={16} />
          Unassigned Users
        </h3>
        <ul className="space-y-1">
          {users.map((user) => (
            <li key={user.id} className="flex items-center gap-2">
              <div className="avatar placeholder">
                <div className="bg-neutral text-neutral-content rounded-full w-7">
                  <span className="text-xs">
                    {user.firstName[0]}
                    {user.lastName[0]}
                  </span>
                </div>
              </div>
              <span className="text-sm">
                {user.firstName} {user.lastName}
              </span>
            </li>
          ))}
        </ul>
      </div>
    </div>
  );
}

function TeamGroup({
  label,
  teams,
  users,
}: {
  label: string;
  teams: Team[];
  users: User[];
}) {
  return (
    <div className="mb-8">
      <h2 className="text-lg font-semibold mb-3 text-base-content/80">{label}</h2>
      <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
        {teams.map((team) => (
          <TeamCard
            key={team.id}
            team={team}
            members={users.filter((u) => u.teamId === team.id)}
          />
        ))}
      </div>
    </div>
  );
}

export default function TeamsSection() {
  const { data: teams = [] } = useTeams();
  const { data: users = [] } = useUsers();
  const { data: businessUnits = [] } = useBusinessUnits();

  const unassignedTeams = teams.filter((t) => t.businessUnitId === null);
  const unassignedUsers = users.filter((u) => u.teamId === -1);

  return (
    <>
      <div className="mb-6">
        <h1 className="text-2xl font-semibold mb-1">Teams</h1>
        <p className="text-base-content/70">Teams and their members</p>
      </div>

      {businessUnits.map((bu) => {
        const buTeams = teams.filter((t) => t.businessUnitId === bu.id);
        if (buTeams.length === 0) return null;
        return <TeamGroup key={bu.id} label={bu.name} teams={buTeams} users={users} />;
      })}

      {unassignedTeams.length > 0 && (
        <TeamGroup label="Unassigned" teams={unassignedTeams} users={users} />
      )}

      {unassignedUsers.length > 0 && (
        <div className="mb-8">
          <h2 className="text-lg font-semibold mb-3 text-base-content/80">Unassigned Users</h2>
          <UnassignedUsersCard users={unassignedUsers} />
        </div>
      )}
    </>
  );
}
